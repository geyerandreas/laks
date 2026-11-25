use crate::settings::BASE_DIRECTORY;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub data_directory: String,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            data_directory: format!("{BASE_DIRECTORY}/storage"),
        }
    }
}
