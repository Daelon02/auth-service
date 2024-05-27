use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectToAuth0<T: Serialize> {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
    pub grant_type: String,
    pub user_id: String,
    pub connection: String,
    #[serde(flatten)]
    pub extra: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationFlow {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginFlow {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePassFlow {
    pub email: String,
}
