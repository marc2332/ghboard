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
                    class: "p-2 text-2xl",
                    "{cx.props.user}"
                }
                h2 {
                    class: "p-2",
                    "Current streak: {cx.props.user_data.current_streak}"
                }
                h2 {
                    class: "p-2",
                    "Longest streak: {cx.props.user_data.longest_streak}"
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
            class: "bg-zinc-900",
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
    if let Some(day) = &cx.props.day {
        let color = match day.contributionCount {
            i if i > 20 => "bg-emerald-300",
            i if i > 10 => "bg-emerald-400",
            i if i > 5 => "bg-emerald-600",
            i if i > 0 => "bg-emerald-800",
            _ => "bg-zinc-950",
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
