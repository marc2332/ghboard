mod dashboard;

use chrono::{Datelike, Utc};
use dashboard::*;

use axum::{extract::Path, response::Html, routing::get, Router};
use dioxus::prelude::*;
use octocrab::Octocrab;

struct AppProps {
    collections: Vec<(ContributionCalendar, i32)>,
    last_year_collection: ContributionCalendar,
    streak: i32,
}

fn app(cx: Scope<AppProps>) -> Element {
    render!(
        script {
            src: "https://cdn.tailwindcss.com"
        }
        div {
            class: "h-full bg-zinc-900 overflow-auto",
            div {
                class: "h-full flex justify-center mx-auto",
                div {
                    h2 {
                        class: "text-white p-2",
                        "Current streak: {cx.props.streak}"
                    }
                    h3 {
                        class: "text-white p-2",
                        "{cx.props.last_year_collection.totalContributions} contributions in the last year"
                    }
                    Calendar {
                        collection: cx.props.last_year_collection.clone(),
                    },
                    for (collection, year) in &cx.props.collections {
                        rsx!(
                            h3 {
                                class: "text-white p-2",
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

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/:user", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}

async fn app_endpoint(Path(user): Path<String>) -> Html<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let client = Octocrab::builder().personal_token(token).build().unwrap();

    let now = Utc::now();
    let joined = get_join_date(&client, &user)
        .await
        .with_month0(0)
        .unwrap()
        .with_day0(0)
        .unwrap();
    let years_since_joined = now.years_since(joined).unwrap() as i32;
    let one_year_ago = now.with_year(now.year() - 1).unwrap();

    let mut years = Vec::new();

    let mut streak = 0;

    for year_num in 0..years_since_joined + 1 {
        let year = joined.year() + year_num;

        let from = new_date_year(year);
        let to = from.with_month(12).unwrap().with_day(31).unwrap();

        let year_data = get_calendar(&client, &user, from, to).await;

        years.insert(0, (year_data, year));
    }

    for (year, year_num) in years.iter().rev() {
        let mut day_c = 0;
        for week in &year.weeks {
            for day in &week.contributionDays {
                if day_c as u32 > now.ordinal0() && *year_num == now.year() {
                    break;
                }

                streak += 1;
                if day.contributionCount == 0 && streak < 500 {
                    streak = 0;
                }

                day_c += 1;
            }
        }
    }

    let last_year_collection = get_calendar(&client, &user, one_year_ago, now).await;

    let mut app = VirtualDom::new_with_props(
        app,
        AppProps {
            collections: years,
            last_year_collection,
            streak,
        },
    );
    let _ = app.rebuild();
    Html(dioxus_ssr::render(&app))
}
