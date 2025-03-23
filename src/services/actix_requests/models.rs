use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisteredUserData {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdatePasswordData {
    pub user_id: String,
    pub email: String,
}

// structs for returning data to client
#[derive(Debug, Clone, Serialize)]
pub struct RegisterUserResponse {
    pub user_id: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginUserResponse {
    pub token: String,
}
