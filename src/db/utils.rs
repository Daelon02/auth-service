use crate::consts::{ACCESS_TOKEN, APPLICATION_JSON, AUDIENCE, CONTENT_TYPE, GRANT_TYPE_PASS};
use crate::errors::{Error, Result};
use crate::models::{ConnectToAuth0, UserData, UserFlow};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use reqwest::{Client, Method};
use serde_json::Value;
use uuid::Uuid;

pub type DatabasePool = Pool<AsyncPgConnection>;

pub async fn create_connection_pool(database_url: String) -> Result<DatabasePool> {
    let manager = AsyncDieselConnectionManager::new(database_url);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}

pub async fn register_user(user: UserData, user_id: Uuid) -> Result<()> {
    let client = Client::new();
    let url = dotenv::var("CLIENT").unwrap_or_else(|_| "localhost:8080".to_string());

    let body = generate_body_for_auth0(user, user_id);

    client
        .request(Method::POST, &url)
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .json(&body)
        .send()
        .await?;
    Ok(())
}

pub async fn get_jwt_user_token(user: UserData) -> Result<String> {
    let client = Client::new();
    let url = dotenv::var("CLIENT").unwrap_or_else(|_| "localhost:8080".to_string());

    let body = generate_body_for_auth0(user, Uuid::new_v4());

    let response = client
        .request(Method::POST, &url)
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .json(&body)
        .send()
        .await?;

    let response: Result<Value> = response.json().await.map_err(Error::from);
    match response {
        Ok(value) => {
            log::info!("Value: {:?}", value);
            let token = value[ACCESS_TOKEN].as_str().ok_or(Error::InvalidToken)?;
            Ok(token.to_string())
        }
        Err(e) => {
            log::error!("Error: {}", e);
            Err(Error::InvalidToken)
        }
    }
}

fn generate_body_for_auth0(user: UserData, user_id: Uuid) -> UserFlow {
    let client_id = dotenv::var("CLIENT_ID").unwrap_or_else(|_| "admin".to_string());
    let client_secret = dotenv::var("CLIENT_SECRET").unwrap_or_else(|_| "admin".to_string());
    let connection = dotenv::var("CONNECTION")
        .unwrap_or_else(|_| "Username-Password-Authentication".to_string());

    UserFlow {
        connect: ConnectToAuth0 {
            client_id,
            client_secret,
            audience: AUDIENCE.to_string(),
            grant_type: GRANT_TYPE_PASS.to_string(),
            user_id: user_id.to_string(),
            connection,
        },
        username: user.username,
        password: user.password,
        email: user.email,
    }
}
