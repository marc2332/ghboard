use std::fs::{self, read_to_string, write};

use crate::client::UserData;

pub struct Cache;

const PATH: &str = "cache";

impl Cache {
    fn init() {
        fs::create_dir(PATH).ok();
    }

    pub fn get(key: &str) -> Option<UserData> {
        Self::init();

        let user_path = format!("{PATH}/{key}");

        let user_data = read_to_string(user_path).ok()?;

        serde_json::from_str(&user_data).ok()
    }

    pub fn set(key: &str, user_data: UserData) {
        Self::init();

        let user_path = format!("{PATH}/{key}");

        let user_data = serde_json::to_string(&user_data).unwrap();

        write(user_path, user_data).unwrap();
    }
}
