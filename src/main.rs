mod cache;
mod client;
mod components;
mod routes;

use std::{collections::HashMap, env};

use cache::Cache;
use chrono::Utc;
use client::get_user_data;
use routes::{
    home::{home_route, HomeRouteProps},
    users::{user_route, UserRouteProps},
};

use std::path::PathBuf;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

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
    token: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()
                .unwrap()
                .add_directive("ghboard=debug".parse().unwrap()),
        )
        .init();

    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").unwrap();

    let state = ApiState { token };

    let router = Router::new()
        .route("/", get(home_endpoint))
        .nest_service("/public", ServeDir::new(PathBuf::from("public")))
        .route("/user/:user", get(user_endpoint))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
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
    let size = params.remove("size").unwrap_or("normal".to_string());
    let key = format!("user_{user}");
    let now = Utc::now();

    let user_data = {
        let cached_user_data = if let Some(user_data) = Cache::get(&key) {
            if now.timestamp_millis() - user_data.created_at.timestamp_millis() < ONE_HOUR {
                info!("Retrieved cached data for user {key}");
                Some(user_data)
            } else {
                info!("Cached data for user {key} is too old");
                None
            }
        } else {
            info!("Cached data of user {key} is invalid");
            None
        };
        if let Some(cached_user_data) = cached_user_data {
            cached_user_data
        } else {
            info!("Fetching data for user {key}");
            let data = get_user_data(&user, state.token).await?;
            Cache::set(&key, data.clone());
            data
        }
    };

    let mut app = VirtualDom::new_with_props(
        user_route,
        UserRouteProps {
            user_data,
            user,
            theme,
            size,
        },
    );
    let _ = app.rebuild();

    Ok(Html(dioxus_ssr::render(&app)))
}
