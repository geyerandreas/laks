use crate::settings::BASE_DIRECTORY;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SQLite {
    pub address: String,
    pub port: u16,
    pub db: String,
    pub user: String,
    pub password: String,
}

impl Default for SQLite {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 0,
            db: format!("sqlite://./{BASE_DIRECTORY}/laks.sqlite?mode=rwc"),
            user: String::new(),
            password: String::new(),
        }
    }
}
