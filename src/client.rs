use std::collections::HashMap;

use axum::response::ErrorResponse;
use chrono::{DateTime, Datelike, SecondsFormat, Timelike, Utc};
use octocrab::{Error, Octocrab};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct ContributionsCollection {
    pub contributionCalendar: ContributionCalendar,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct ContributionCalendar {
    pub totalContributions: usize,
    pub weeks: Vec<GhWeek>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct GhWeek {
    pub contributionDays: Vec<GhDay>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct GhDay {
    pub contributionCount: i32,
    pub weekday: usize,
    pub date: String,
}

pub fn new_date_year(year: i32) -> DateTime<Utc> {
    Utc::now()
        .with_day0(0)
        .unwrap()
        .with_month0(0)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_year(year)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

pub fn date_with_just_year(date: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date.with_month0(0)?
        .with_day0(0)?
        .with_hour(0)?
        .with_minute(0)?
        .with_second(0)
}

pub async fn get_join_date(client: &Octocrab, user: &str) -> Result<DateTime<Utc>, Error> {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Response {
        data: Data,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Data {
        user: User,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[allow(non_snake_case)]
    struct User {
        createdAt: DateTime<Utc>,
    }

    let response = client
        .graphql::<Response>(&json!({
            "query":
                format!(
                    "
    {{
        user(login: \"{user}\") {{
            createdAt
        }}
      }}
    "
                )
        }))
        .await?;

    Ok(response.data.user.createdAt)
}

pub async fn get_calendars(
    client: &Octocrab,
    user: &str,
    calendars: &[(String, DateTime<Utc>, DateTime<Utc>)],
) -> HashMap<String, ContributionsCollection> {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Response {
        data: Data,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Data {
        user: HashMap<String, ContributionsCollection>,
    }

    let mut queries = String::new();

    for (key, from, to) in calendars {
        let from = from.to_rfc3339_opts(SecondsFormat::Secs, true);
        let to = to.to_rfc3339_opts(SecondsFormat::Secs, true);
        queries.push_str(&format!(
            "
        {key}: contributionsCollection(from: \"{from}\", to: \"{to}\") {{
            contributionCalendar {{
              totalContributions
              weeks {{
                contributionDays {{
                  contributionCount
                  weekday
                  date
                }}
              }}
            }}
          }}
          \n
        "
        ))
    }

    let req = format!(
        "
        {{
            user(login: \"{user}\") {{
                {queries}
            }}
        }}
"
    );

    let response = client
        .graphql::<Response>(&json!({ "query": req }))
        .await
        .unwrap();

    response.data.user
}

pub fn get_streaks(years: &[(ContributionsCollection, i32)], now: DateTime<Utc>) -> (i32, i32) {
    let mut current_streak = 0;
    let mut longest_streak = 0;

    'counter: for (year_i, (year, _)) in years.iter().rev().enumerate() {
        let is_last_year = year_i == years.len() - 1;
        let mut day_c = 0;
        for week in &year.contributionCalendar.weeks {
            for day in &week.contributionDays {
                let is_last_day = day_c == now.ordinal0() && is_last_year;
                let is_future = day_c >= now.ordinal0() && is_last_year;

                // Count contributions
                if day.contributionCount > 0 {
                    current_streak += 1;
                }

                // keep longest streak
                if current_streak > longest_streak {
                    longest_streak = current_streak;
                }

                if is_future {
                    break 'counter;
                }

                // Reset contributions
                if day.contributionCount == 0 && !is_last_day {
                    current_streak = 0;
                }

                day_c += 1;
            }
        }
    }

    (current_streak, longest_streak)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub created_at: DateTime<Utc>,
    pub years: Vec<(ContributionsCollection, i32)>,
    pub last_year: ContributionsCollection,
    pub current_streak: i32,
    pub longest_streak: i32,
}

pub async fn get_user_data(user: &str, token: String) -> Result<UserData, ErrorResponse> {
    let client = Octocrab::builder().personal_token(token).build().unwrap();

    let now = Utc::now();
    let joined = date_with_just_year(
        get_join_date(&client, user)
            .await
            .map_err(|_| ErrorResponse::from("Something went wrong, try again."))?,
    )
    .unwrap();
    let years_since_joined = date_with_just_year(now)
        .unwrap()
        .years_since(joined)
        .unwrap() as i32;
    let one_year_ago = now.with_year(now.year() - 1).unwrap();
    let years_range = 0..years_since_joined + 1;

    // Create calendar queries

    let mut calendars = years_range
        .clone()
        .map(|year_num| {
            let year = joined.year() + year_num;
            let from = new_date_year(year);
            let to = from.with_month(12).unwrap().with_day(31).unwrap();
            let key = format!("year{}{}", from.timestamp(), to.timestamp());

            (key, from, to)
        })
        .collect::<Vec<(String, DateTime<Utc>, DateTime<Utc>)>>();

    calendars.push(("past_year".to_string(), one_year_ago, now));

    let mut calendars_results = get_calendars(&client, user, &calendars).await;

    // Collect calendars results

    let calendars_data = years_range
        .rev()
        .map(|year_num| {
            let year = joined.year() + year_num;
            let from = new_date_year(year);
            let to = from.with_month(12).unwrap().with_day(31).unwrap();
            let key = format!("year{}{}", from.timestamp(), to.timestamp());

            let year_data = calendars_results.remove(&key).unwrap();

            (year_data, year)
        })
        .collect::<Vec<(ContributionsCollection, i32)>>();

    let (current_streak, longest_streak) = get_streaks(&calendars_data, now);

    let last_year = calendars_results.remove("past_year").unwrap();

    Ok(UserData {
        years: calendars_data,
        last_year,
        current_streak,
        longest_streak,
        created_at: Utc::now(),
    })
}
