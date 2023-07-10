use axum::response::ErrorResponse;
use chrono::{DateTime, Datelike, Utc};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

use crate::client::{
    date_with_just_year, get_calendars, get_join_date, get_streaks, new_date_year,
    ContributionsCollection,
};

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
    let years_since_joined = now.years_since(joined).unwrap() as i32;
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
