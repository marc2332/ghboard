use axum::response::ErrorResponse;
use chrono::{DateTime, Datelike, Utc};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

use crate::client::{get_calendar, get_join_date, get_streak, new_date_year, ContributionCalendar};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub created_at: DateTime<Utc>,
    pub years: Vec<(ContributionCalendar, i32)>,
    pub last_year: ContributionCalendar,
    pub streak: i32,
}

pub async fn get_user_data(user: &str, token: String) -> Result<UserData, ErrorResponse> {
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
