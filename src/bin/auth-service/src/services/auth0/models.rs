use crate::errors::Result;
use crate::services::auth0::errors::BuildError;
use builder_derive::Builder;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Builder)]
pub struct Auth0Request<T: Serialize + DeserializeOwned + Debug + Default> {
    client_id: String,
    client_secret: String,
    audience: String,
    grant_type: String,
    user_id: String,
    connection: String,
    scope: String,
    extra: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct SignupRequest {
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub connection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct SignInRequest {
    client_id: String,
    client_secret: String,
    audience: String,
    grant_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistrationFlow {
    pub client_id: String,
    pub client_secret: String,
    pub password: String,
    pub email: String,
    pub connection: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginFlow {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangePassFlow {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth0RegisterResponse {
    pub _id: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisteredUserResponse {
    pub id: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Auth0LoginResponse {
    pub access_token: String,
    pub id_token: String,
    pub expires_in: u64,
    pub token_type: String,
}
