use dioxus::prelude::*;

use crate::{
    cache::UserData,
    client::{ContributionsCollection, GhDay, GhWeek},
    components::page::Page,
};

pub struct UserRouteProps {
    pub user_data: UserData,
    pub user: String,
    pub theme: String,
}

pub fn user_route(cx: Scope<UserRouteProps>) -> Element {
    render!(
        Page {
            title: "{cx.props.user} | ghboard",
            theme: &cx.props.theme,
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
                    ThemeLink { code: "github", name: "🐙 GitHub" },
                    ThemeLink { code: "winter", name: "🥶 Winter" },
                    ThemeLink { code: "halloween", name: "🎃 Halloween" },
                    ThemeLink { code: "barbie", name: "👸 Barbie" },
                    ThemeLink { code: "oppenheimer", name: "💣 Oppenheimer" },
                }
                h4 {
                    class: "data-title",
                    "{cx.props.user_data.last_year.contributionCalendar.totalContributions} contributions in the last year"
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
    render!(
        div {
            class: "calendar",
            for week in &collection.contributionCalendar.weeks {
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
pub fn Week(cx: Scope, week: GhWeek) -> Element {
    render!(
        div {
            class: "calendar-week",
            for day_n in 0..7 {
                if let Some(day) = week.contributionDays.iter().find(|day| day.weekday == day_n).cloned() {
                    rsx!(
                        Day {
                            day: day
                        }
                    )
                } else {
                    rsx!(
                        Day { }
                    )
                }
            }
        }
    )
}

#[derive(Props, PartialEq)]
pub struct DayProps {
    day: Option<GhDay>,
}

#[allow(non_snake_case)]
pub fn Day(cx: Scope<DayProps>) -> Element {
    if let Some(day) = &cx.props.day {
        let color_class = match day.contributionCount {
            i if i > 20 => "quiteALot",
            i if i > 10 => "aLot",
            i if i > 5 => "okay",
            i if i > 0 => "meh",
            _ => "nothing",
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
fn ThemeLink<'a>(cx: Scope, code: &'a str, name: &'a str) -> Element {
    render!(
        a {
            class: "theme-link",
            href: "?theme={code}",
            "{name}"
        }
    )
}
