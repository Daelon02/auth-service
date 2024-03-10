use crate::errors::{Error, Result};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use reqwest::{Client, Method};
use serde_json::Value;

pub type DatabasePool = Pool<AsyncPgConnection>;

pub async fn create_connection_pool(database_url: String) -> Result<DatabasePool> {
    let manager = AsyncDieselConnectionManager::new(database_url);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}

pub async fn get_jwt_token() -> Result<String> {
    let client = Client::new();
    let url = dotenv::var("CLIENT").unwrap_or_else(|_| "localhost:8080".to_string());
    let client_id = dotenv::var("CLIENT_ID").unwrap_or_else(|_| "admin".to_string());
    let client_secret = dotenv::var("CLIENT_SECRET").unwrap_or_else(|_| "admin".to_string());
    let request = client
        .request(Method::POST, &url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "client_id": client_id,
            "client_secret": client_secret,
            "audience":"https://someexample.com",
            "grant_type":"client_credentials"
        }))
        .send()
        .await?;
    let response = request.json::<Value>().await?;
    log::info!("Response: {:?}", response);
    let token = match response["access_token"].as_str() {
        Some(token) => token,
        None => return Err(Error::InvalidInput("Invalid token".to_string())),
    };
    Ok(token.to_string())
}
