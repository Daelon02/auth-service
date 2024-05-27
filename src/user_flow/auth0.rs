use crate::consts::{ACCESS_TOKEN, APPLICATION_JSON, CONTENT_TYPE};
use crate::errors::Error;
use crate::models::{RegisteredUserData, UserData};
use crate::services::auth0::Auth0Service;
use crate::user_flow::consts::{LOGIN_URL, REGISTRATION_URL};
use actix_web::web::Data;
use http::Method;
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

pub async fn register_user(
    user: UserData,
    user_id: Uuid,
    auth0_service: Data<Auth0Service>,
) -> crate::errors::Result<()> {
    let client = Client::new();

    let url = format!("{}{}", auth0_service.client_url, REGISTRATION_URL);

    let body = auth0_service.generate_body_for_registration(
        user_id,
        user.username,
        user.password,
        user.email,
    );

    client
        .request(Method::POST, &url)
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .json(&body)
        .send()
        .await?;
    Ok(())
}

pub async fn get_jwt_user_token(
    user: RegisteredUserData,
    auth0_service: Data<Auth0Service>,
) -> crate::errors::Result<String> {
    let client = Client::new();

    let url = format!("{}{}", auth0_service.client_url, LOGIN_URL);

    let body = auth0_service
        .generate_body_for_login(user.id, user.username, user.password)
        .await;

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
