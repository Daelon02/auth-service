use crate::consts::{APPLICATION_JSON, AUTHORIZATION, CONTENT_TYPE};
use crate::errors::{Error, Result};
use crate::services::auth0::Auth0Service;
use crate::user_flow::consts::{CHANGE_PASSWORD_URL, GET_PROFILE_URL};
use actix_web::web::Data;
use http::Method;
use reqwest::Client;
use uuid::Uuid;

pub async fn send_request_to_change_pass(
    user_id: Uuid,
    email: String,
    auth0_service: Data<Auth0Service>,
) -> Result<()> {
    let client = Client::new();

    let url = format!("{}/{}", auth0_service.client_url, CHANGE_PASSWORD_URL);

    let body = auth0_service
        .generate_body_for_change_password(user_id, email)
        .await;

    client
        .request(Method::POST, &url)
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .json(&body)
        .send()
        .await?;

    Ok(())
}

pub async fn send_request_to_get_profile(
    access_token: &str,
    auth0_service: Data<Auth0Service>,
) -> Result<String> {
    let client = Client::new();

    let url = format!("{}/{}", auth0_service.client_url, GET_PROFILE_URL);
    log::info!("Requesting profile from URL: {}", &url);

    let result = client
        .get(&url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await;

    match result {
        Ok(response) => {
            log::info!("Response status: {}", response.status());
            if response.status().is_success() {
                let body = response.text().await?;
                log::info!("Response body: {:?}", body);
                Ok(body)
            } else {
                let error_body = response.text().await?;
                log::error!("Error response body: {:?}", error_body);
                Err(Error::StringError(error_body))
            }
        }
        Err(e) => {
            log::error!("Request failed: {:?}", e);
            Err(Error::StringError(e.to_string()))
        }
    }
}
