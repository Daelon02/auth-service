use crate::consts::{APPLICATION_JSON, CONTENT_TYPE};
use crate::errors::Result;
use crate::services::auth0::Auth0Service;
use crate::user_flow::consts::CHANGE_PASSWORD_URL;
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

    let url = format!("{}{}", auth0_service.client_url, CHANGE_PASSWORD_URL);

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
