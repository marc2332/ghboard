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

pub async fn get_calendar(
    client: &Octocrab,
    user: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> ContributionCalendar {
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
        contributionsCollection: ContributionsCollection,
    }

    let from = from.to_rfc3339_opts(SecondsFormat::Secs, true);
    let to = to.to_rfc3339_opts(SecondsFormat::Secs, true);

    let response = client
        .graphql::<Response>(&json!({
            "query":
                format!(
                    "
    {{
        user(login: \"{user}\") {{
          contributionsCollection(from: \"{from}\", to: \"{to}\") {{
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
        }}
      }}
    "
                )
        }))
        .await
        .unwrap();

    response
        .data
        .user
        .contributionsCollection
        .contributionCalendar
}

pub fn get_streak(years: &[(ContributionCalendar, i32)], now: DateTime<Utc>) -> i32 {
    let mut streak = 0;

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

    streak
}
