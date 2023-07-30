use dioxus::prelude::*;

use crate::{
    cache::UserData,
    client::{ContributionsCollection, GhDay, GhWeek},
    components::page::Page,
};

pub struct UserRouteProps {
    pub user_data: UserData,
    pub user: String,
}

pub fn user_route(cx: Scope<UserRouteProps>) -> Element {
    render!(
        Page {
            title: "{cx.props.user} | ghboard",
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
    let theme = use_context::<Theme>(cx).unwrap();
    if let Some(day) = &cx.props.day {
        let color = match day.contributionCount {
            i if i > 20 => &theme.quite_a_lot,
            i if i > 10 => &theme.a_lot,
            i if i > 5 => &theme.okay,
            i if i > 0 => &theme.meh,
            _ => &theme.nothing,
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
            style: "background-color: {color};",
            class: "calendar-day",
            title: "{day.contributionCount} contributions on {day_name}, {day.date}"
        })
    } else {
        render!(div {
            class: "calendar-day",
        })
    }
}

#[derive(Clone)]
pub struct Theme {
    quite_a_lot: String,
    a_lot: String,
    okay: String,
    meh: String,
    nothing: String,
}

impl Theme {
    pub fn from_name(name: &str) -> Theme {
        match name {
            "halloween" => Theme {
                quite_a_lot: "rgb(253 224 71)".to_string(),
                a_lot: "rgb(250 204 21)".to_string(),
                okay: "rgb(202 138 4)".to_string(),
                meh: "rgb(133 77 14)".to_string(),
                nothing: "rgb(9 9 11)".to_string(),
            },
            "winter" => Theme {
                quite_a_lot: "rgb(191 219 254)".to_string(),
                a_lot: "rgb(96 165 250)".to_string(),
                okay: "rgb(29 78 216)".to_string(),
                meh: "rgb(30 58 138)".to_string(),
                nothing: "rgb(9 9 11)".to_string(),
            },
            "barbie" => Theme {
                quite_a_lot: "rgb(251 207 232)".to_string(),
                a_lot: "rgb(244 114 182)".to_string(),
                okay: "rgb(190 24 93)".to_string(),
                meh: "rgb(131 24 67)".to_string(),
                nothing: "rgb(9 9 11)".to_string(),
            },
            "oppenheimer" => Theme {
                quite_a_lot: "rgb(254 249 195)".to_string(),
                a_lot: "rgb(251 191 36)".to_string(),
                okay: "rgb(194 65 12)".to_string(),
                meh: "rgb(120 53 15)".to_string(),
                nothing: "rgb(9 9 11)".to_string(),
            },
            _ => Theme {
                quite_a_lot: "rgb(110 231 183)".to_string(),
                a_lot: "rgb(52 211 153)".to_string(),
                okay: "rgb(5 150 105)".to_string(),
                meh: "rgb(6 95 70)".to_string(),
                nothing: "rgb(9 9 11)".to_string(),
            },
        }
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
