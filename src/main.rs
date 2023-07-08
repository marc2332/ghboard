mod dashboard;

use chrono::{DateTime, Datelike, Utc};
use dashboard::*;
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use tracing::info;

use axum::{
    extract::{Path, State},
    response::{self, ErrorResponse, Html},
    routing::get,
    Router,
};
use dioxus::prelude::*;
use dotenv::dotenv;
use octocrab::Octocrab;

struct AppProps {
    user_data: UserData,
    user: String,
}

fn app(cx: Scope<AppProps>) -> Element {
    render!(
        head {
            title {
                "{cx.props.user} | ghboard"
            }
            link {
                rel: "icon",
                href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🦑</text></svg>"
            }
        }
        script {
            src: "https://cdn.tailwindcss.com"
        }
        div {
            class: "h-full bg-zinc-900 overflow-auto text-white",
            div {
                class: "h-full flex justify-center mx-auto",
                div {
                    h1 {
                        class: "p-2 text-2xl",
                        "{cx.props.user}"
                    }
                    h2 {
                        class: "p-2",
                        "Current streak: {cx.props.user_data.streak}"
                    }
                    b {
                        class: "p-2",
                        "Don't forget to star the ",
                        a {
                            class: "underline",
                            href: "https://github.com/marc2332/ghboard",
                            "repository ⭐😄"
                        }
                    }
                    h3 {
                        class: "p-2",
                        "{cx.props.user_data.last_year.totalContributions} contributions in the last year"
                    }
                    Calendar {
                        collection: cx.props.user_data.last_year.clone(),
                    },
                    for (collection, year) in &cx.props.user_data.years {
                        rsx!(
                            h3 {
                                class: "p-2",
                                "{collection.totalContributions} contributions in {year}"
                            }
                            Calendar {
                                collection: collection.clone(),
                            }
                        )
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Calendar(cx: Scope, collection: ContributionCalendar) -> Element {
    render!(
        div {
            class: "bg-zinc-900",
            for week in &collection.weeks {
                rsx!(
                    Week {
                        week: week.clone()
                    }
                )
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Week(cx: Scope, week: GhWeek) -> Element {
    render!(
        div {
            class: "w-[15px] inline-block",
            for day in &week.contributionDays {
                rsx!(
                    Day {
                        day: day.clone()
                    }
                )
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Day(cx: Scope, day: GhDay) -> Element {
    let color = match day.contributionCount {
        i if i > 20 => "bg-emerald-300",
        i if i > 10 => "bg-emerald-400",
        i if i > 5 => "bg-emerald-600",
        i if i > 0 => "bg-emerald-800",
        _ => "bg-zinc-950",
    };

    render!(div {
        class: "{color} w-[10px] h-[10px] m-2 rounded-sm",
        title: "{day.contributionCount} {day.weekday} {day.date}"
    })
}

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
        .route("/user/:user", get(app_endpoint))
        .with_state(state);

    Ok(router.into())
}

#[derive(Serialize, Deserialize, Clone)]
struct UserData {
    created_at: DateTime<Utc>,
    years: Vec<(ContributionCalendar, i32)>,
    last_year: ContributionCalendar,
    streak: i32,
}

async fn get_user_data(user: &str, token: String) -> Result<UserData, ErrorResponse> {
    let client = Octocrab::builder().personal_token(token).build().unwrap();

    let now = Utc::now();
    let joined = get_join_date(&client, user)
        .await
        .map_err(|_| ErrorResponse::from("Something went wrong, try again."))?
        .with_month0(0)
        .unwrap()
        .with_day0(0)
        .unwrap();
    let years_since_joined = now.years_since(joined).unwrap() as i32;
    let one_year_ago = now.with_year(now.year() - 1).unwrap();

    let mut years = Vec::new();

    for year_num in 0..years_since_joined + 1 {
        let year = joined.year() + year_num;

        let from = new_date_year(year);
        let to = from.with_month(12).unwrap().with_day(31).unwrap();

        let year_data = get_calendar(&client, user, from, to).await;

        years.insert(0, (year_data, year));
    }

    let streak = get_streak(&years, now);

    let last_year = get_calendar(&client, user, one_year_ago, now).await;

    Ok(UserData {
        years,
        last_year,
        streak,
        created_at: Utc::now(),
    })
}

const ONE_HOUR: i64 = 60 * 60 * 1000;

async fn app_endpoint(
    State(state): State<ApiState>,
    Path(user): Path<String>,
) -> response::Result<Html<String>> {
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
        info!("Cached data not found for user {key} is too old");
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
        app,
        AppProps {
            user_data: new_user_data.unwrap(),
            user,
        },
    );
    let _ = app.rebuild();

    Ok(Html(dioxus_ssr::render(&app)))
}
