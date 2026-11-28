use laks_common::account_name::UserName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub admin_name: UserName,
    pub admin_password: String,
    pub admin_email: String,
}

impl Default for Setup {
    fn default() -> Self {
        Self {
            admin_name: UserName::from_unchecked("admin".to_owned()),
            admin_password: std::env::var("LAKS_ADMIN_PASSWORD").unwrap_or("password".to_owned()),
            admin_email: "admin@example.com".to_owned(),
        }
    }
}
