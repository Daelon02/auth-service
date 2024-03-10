use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFlow {
    #[serde(flatten)]
    pub connect: ConnectToAuth0,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectToAuth0 {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
    pub grant_type: String,
    pub user_id: String,
    pub connection: String,
}
