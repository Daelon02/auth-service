use crate::consts::{ACCESS_TOKEN, APPLICATION_JSON, AUDIENCE, CONTENT_TYPE, GRANT_TYPE_PASS};
use crate::errors::Error;
use crate::models::{ConnectToAuth0, LoginFlow, RegisteredUserData, RegistrationFlow, UserData};
use crate::user_flow::consts::{LOGIN_URL, REGISTRATION_URL};
use actix_web::http::Method;
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

pub async fn register_user(user: UserData, user_id: Uuid) -> crate::errors::Result<()> {
    let client = Client::new();
    let client_url = dotenv::var("CLIENT").unwrap_or_else(|_| "localhost:8080".to_string());

    let url = format!("{}{}", client_url, REGISTRATION_URL);

    let body = generate_body_for_registration(user_id, user.username, user.password, user.email);

    client
        .request(Method::POST, &url)
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .json(&body)
        .send()
        .await?;
    Ok(())
}

pub async fn get_jwt_user_token(user: RegisteredUserData) -> crate::errors::Result<String> {
    let client = Client::new();
    let client_url = dotenv::var("CLIENT").unwrap_or_else(|_| "localhost:8080".to_string());

    let url = format!("{}{}", client_url, LOGIN_URL);

    let body = generate_body_for_login(user.id, user.username, user.password).await;

    let response = client
        .request(Method::POST, &url)
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .json(&body)
        .send()
        .await?;

    let response: crate::errors::Result<Value> = response.json().await.map_err(Error::from);
    match response {
        Ok(value) => {
            let token = value[ACCESS_TOKEN].as_str().ok_or(Error::InvalidToken)?;
            Ok(token.to_string())
        }
        Err(e) => {
            log::error!("Error: {}", e);
            Err(Error::InvalidToken)
        }
    }
}

fn generate_body_for_registration(
    user_id: Uuid,
    username: String,
    password: String,
    email: String,
) -> ConnectToAuth0<RegistrationFlow> {
    let client_id = dotenv::var("CLIENT_ID").unwrap_or_else(|_| "admin".to_string());
    let client_secret = dotenv::var("CLIENT_SECRET").unwrap_or_else(|_| "admin".to_string());
    let connection = dotenv::var("CONNECTION")
        .unwrap_or_else(|_| "Username-Password-Authentication".to_string());

    ConnectToAuth0 {
        client_id,
        client_secret,
        audience: AUDIENCE.to_string(),
        grant_type: GRANT_TYPE_PASS.to_string(),
        user_id: user_id.to_string(),
        connection,
        extra: RegistrationFlow {
            username,
            password,
            email,
        },
    }
}

pub async fn generate_body_for_login(
    user_id: Uuid,
    username: String,
    password: String,
) -> ConnectToAuth0<LoginFlow> {
    let client_id = dotenv::var("CLIENT_ID").unwrap_or_else(|_| "admin".to_string());
    let client_secret = dotenv::var("CLIENT_SECRET").unwrap_or_else(|_| "admin".to_string());
    let connection = dotenv::var("CONNECTION")
        .unwrap_or_else(|_| "Username-Password-Authentication".to_string());

    ConnectToAuth0 {
        client_id,
        client_secret,
        audience: AUDIENCE.to_string(),
        grant_type: GRANT_TYPE_PASS.to_string(),
        user_id: user_id.to_string(),
        connection,
        extra: LoginFlow { username, password },
    }
}
