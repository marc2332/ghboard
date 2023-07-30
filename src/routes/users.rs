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
                class: "pt-4",
                h1 {
                    class: "p-2 text-2xl",
                    "{cx.props.user}",
                    b {
                        class: "p-2 text-base",
                        "Don't forget to star the ",
                        a {
                            class: "underline",
                            href: "https://github.com/marc2332/ghboard",
                            "repository ⭐😄"
                        }
                    }
                }
                h2 {
                    class: "p-2",
                    "Current streak: {cx.props.user_data.current_streak}"
                }
                h2 {
                    class: "p-2",
                    "Longest streak: {cx.props.user_data.longest_streak}"
                }
                div {
                    class: "p-2",
                    span {
                        "Themes: "
                    }
                    ThemeLink { code: "github", name: "🐙 GitHub" },
                    ThemeLink { code: "winter", name: "🥶 Winter" },
                    ThemeLink { code: "halloween", name: "🎃 Halloween" },
                    ThemeLink { code: "barbie", name: "👸 Barbie" },
                    ThemeLink { code: "oppenheimer", name: "💣 Oppenheimer" },
                }
                h3 {
                    class: "p-2",
                    "{cx.props.user_data.last_year.contributionCalendar.totalContributions} contributions in the last year"
                }
                Calendar {
                    collection: cx.props.user_data.last_year.clone(),
                },
                for (collection, year) in &cx.props.user_data.years {
                    rsx!(
                        h3 {
                            class: "p-2",
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
            class: "bg-zinc-900 whitespace-nowrap overflow-auto md:overflow-visible",
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
            class: "w-[15px] inline-block",
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
            class: "{color} w-[10px] h-[10px] m-2 rounded-sm",
            title: "{day.contributionCount} contributions on {day_name}, {day.date}"
        })
    } else {
        render!(div {
            class: "w-[10px] h-[10px] m-2",
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
                quite_a_lot: "bg-yellow-300".to_string(),
                a_lot: "bg-yellow-400".to_string(),
                okay: "bg-yellow-600".to_string(),
                meh: "bg-yellow-800".to_string(),
                nothing: "bg-zinc-950".to_string(),
            },
            "winter" => Theme {
                quite_a_lot: "bg-blue-200".to_string(),
                a_lot: "bg-blue-400".to_string(),
                okay: "bg-blue-700".to_string(),
                meh: "bg-blue-900".to_string(),
                nothing: "bg-zinc-950".to_string(),
            },
            "barbie" => Theme {
                quite_a_lot: "bg-pink-200".to_string(),
                a_lot: "bg-pink-400".to_string(),
                okay: "bg-pink-700".to_string(),
                meh: "bg-pink-900".to_string(),
                nothing: "bg-zinc-950".to_string(),
            },
            "oppenheimer" => Theme {
                quite_a_lot: "bg-yellow-100".to_string(),
                a_lot: "bg-amber-400".to_string(),
                okay: "bg-orange-700".to_string(),
                meh: "bg-amber-900".to_string(),
                nothing: "bg-zinc-950".to_string(),
            },
            _ => Theme {
                quite_a_lot: "bg-emerald-300".to_string(),
                a_lot: "bg-emerald-400".to_string(),
                okay: "bg-emerald-600".to_string(),
                meh: "bg-emerald-800".to_string(),
                nothing: "bg-zinc-950".to_string(),
            },
        }
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn ThemeLink<'a>(cx: Scope, code: &'a str, name: &'a str) -> Element {
    render!(
        a {
            class: "underline mr-2",
            href: "?theme={code}",
            "{name}"
        }
    )
}
