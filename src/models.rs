use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UserDetails {
    pub name: String,
}