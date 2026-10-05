use dioxus::prelude::*;

use crate::{
    client::{ContributionsCollection, GhDay, GhWeek, UserData},
    components::page::Page,
};

pub struct UserRouteProps {
    pub user_data: UserData,
    pub user: String,
    pub theme: String,
    pub size: String,
}

pub fn user_route(cx: Scope<UserRouteProps>) -> Element {
    render!(
        Page {
            title: "{cx.props.user} | ghboard",
            theme: &cx.props.theme,
            size: &cx.props.size,
            div {
                h1 {
                    class: "title",
                    "{cx.props.user}",
                    b {
                        class: "subtitle aligned-text",
                        "Don't forget to star the ",
                        a {
                            href: "https://github.com/marc2332/ghboard",
                            "repository ⭐😄"
                        }
                    }
                }
                h4 {
                    class: "data-title",
                    "Current streak: {cx.props.user_data.current_streak}"
                }
                h4 {
                    class: "data-title",
                    "Longest streak: {cx.props.user_data.longest_streak}"
                }
                div {
                    class: "data-title",
                    span {
                        "Themes: "
                    }
                    CustomizationLink { theme: "github", size: &cx.props.size, text: "🐙 GitHub" },
                    CustomizationLink { theme: "rust", size: &cx.props.size, text: "🦀 Rust" },
                    CustomizationLink { theme: "javascript", size: &cx.props.size, text: "🟨 JavaScript" },
                    CustomizationLink { theme: "go", size: &cx.props.size, text: "🐹 Go" },
                    CustomizationLink { theme: "mono", size: &cx.props.size, text: "⚪ Mono" },
                    CustomizationLink { theme: "linux", size: &cx.props.size, text: "🐧 Linux" },
                }
                div {
                    class: "data-title",
                    span {
                        "Sizes: "
                    }
                    CustomizationLink { theme: &cx.props.theme, size: "fully-compact", text: "🤏 Fully Compact" },
                    CustomizationLink { theme: &cx.props.theme, size: "compact", text: "📦 Compact" },
                    CustomizationLink { theme: &cx.props.theme, size: "normal", text: "👍 Normal" },
                }
                h4 {
                    class: "data-title",
                    "{cx.props.user_data.last_year.contributionCalendar.totalContributions} contributions in the last 365 days"
                }
                Calendar {
                    collection: cx.props.user_data.last_year.clone(),
                },
                for (collection, year) in &cx.props.user_data.years {
                    rsx!(
                        h4 {
                            class: "data-title",
                            "{collection.contributionCalendar.totalContributions} contributions in {year}"
                        }
                        Calendar {
                            collection: collection.clone(),
                        }
                    )
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn Calendar(cx: Scope, collection: ContributionsCollection) -> Element {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut total_contributions = 0.0;
    let mut day_count = 0;
    for day in collection
        .contributionCalendar
        .weeks
        .iter()
        .flat_map(|week| &week.contributionDays)
    {
        if day.date <= today {
            total_contributions += f64::from(day.contributionCount);
            day_count += 1;
        }
    }
    let average = if day_count > 0 {
        total_contributions / f64::from(day_count)
    } else {
        0.0
    };

    render!(
        div {
            class: "calendar",
            for week in &collection.contributionCalendar.weeks {
                rsx!(
                    Week {
                        week: week.clone(),
                        average: average,
                    }
                )
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn Week(cx: Scope, week: GhWeek, average: f64) -> Element {
    render!(
        div {
            class: "calendar-week",
            for day_n in 0..7 {
                if let Some(day) = week.contributionDays.iter().find(|day| day.weekday == day_n).cloned() {
                    rsx!(
                        Day {
                            day: day,
                            average: *average,
                        }
                    )
                } else {
                    rsx!(
                        Day { average: *average }
                    )
                }
            }
        }
    )
}

#[derive(Props, PartialEq)]
pub struct DayProps {
    day: Option<GhDay>,
    average: f64,
}

#[allow(non_snake_case)]
pub fn Day(cx: Scope<DayProps>) -> Element {
    if let Some(day) = &cx.props.day {
        let relative_count = if cx.props.average > 0.0 {
            f64::from(day.contributionCount) / cx.props.average
        } else {
            0.0
        };
        let intensity = relative_count.max(0.0) / (relative_count.max(0.0) + 1.0);
        let shade = (intensity * 63.0).round();
        let shade_fraction = shade / 63.0;
        let color_class = if day.contributionCount > 0 {
            "active"
        } else {
            "nothing"
        };

        let day_name = match day.weekday {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => "Sunday",
        };

        render!(div {
            class: "calendar-day {color_class}",
            style: "--contribution-shade: {shade_fraction}",
            title: "{day.contributionCount} contributions on {day_name}, {day.date}"
        })
    } else {
        render!(div {
            class: "calendar-day",
        })
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn CustomizationLink<'a>(cx: Scope, theme: &'a str, size: &'a str, text: &'a str) -> Element {
    render!(
        a {
            class: "customization-link",
            href: "?theme={theme}&size={size}",
            "{text}"
        }
    )
}
