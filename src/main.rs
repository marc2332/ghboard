mod cache;
mod client;
mod components;
mod routes;

use std::collections::HashMap;

use cache::{get_user_data, UserData};
use chrono::Utc;
use routes::{
    home::{home_route, HomeRouteProps},
    users::{user_route, UserRouteProps},
};
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use tracing::info;

use axum::{
    extract::{Path, Query, State},
    response::{self, Html},
    routing::get,
    Router,
};
use dioxus::prelude::*;
use dotenv::dotenv;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct ApiState {
    persist: PersistInstance,
    token: String,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_axum::ShuttleAxum {
    dotenv().ok();
    let token = secret_store
        .get("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN env variable is required");

    let state = ApiState { token, persist };

    let router = Router::new()
        .route("/", get(home_endpoint))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/user/:user", get(user_endpoint))
        .with_state(state);

    Ok(router.into())
}

async fn home_endpoint() -> response::Result<Html<String>> {
    let mut app = VirtualDom::new_with_props(home_route, HomeRouteProps {});
    let _ = app.rebuild();

    Ok(Html(dioxus_ssr::render(&app)))
}

const ONE_HOUR: i64 = 60 * 60 * 1000;

async fn user_endpoint(
    State(state): State<ApiState>,
    Path(user): Path<String>,
    Query(mut params): Query<HashMap<String, String>>,
) -> response::Result<Html<String>> {
    let theme = params.remove("theme").unwrap_or("github".to_string());
    let key = format!("user_{user}");
    let user_data = state.persist.load::<UserData>(&key);
    let now = Utc::now();

    let mut new_user_data: Option<UserData> = None;

    if let Ok(user_data) = user_data {
        if now.timestamp_millis() - user_data.created_at.timestamp_millis() < ONE_HOUR {
            new_user_data = Some(user_data);
            info!("Retrieved cached data for user {key}");
        } else {
            info!("Cached data for user {key} is too old");
        }
    } else {
        info!("Cached data of user {key} is invalid");
    }

    if new_user_data.is_none() {
        let data = get_user_data(&user, state.token).await?;
        if let Err(err) = state.persist.save(&key, data.clone()) {
            info!("Failed caching data for user {key}: {err}");
        } else {
            info!("Cached data for user {key}");
        }
        new_user_data = Some(data)
    }

    let mut app = VirtualDom::new_with_props(
        user_route,
        UserRouteProps {
            user_data: new_user_data.unwrap(),
            user,
            theme,
        },
    );
    let _ = app.rebuild();

    Ok(Html(dioxus_ssr::render(&app)))
}
