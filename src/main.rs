pub mod actors;
pub mod consts;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod services;
pub mod user_flow;
pub mod utils;

use crate::db::postgres_db::DbService;
use crate::errors::Result;
use crate::utils::{configure_data, configure_routes, init_logging};
use actix::Actor;

use crate::services::auth0::Auth0Service;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::user_flow::requests::login,
        crate::user_flow::requests::register,
        crate::user_flow::requests::change_password
    ),
    components(
        schemas(crate::models::RegisteredUserData),
        schemas(crate::models::UserData),
        schemas(crate::models::UpdatePasswordData),
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<()> {
    init_logging()?;
    log::info!("Starting auth service...");

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::utils::create_connection_pool(database_url).await?;
    let db = DbService::new(pool);
    let db = db.start();

    let client_id = dotenv::var("CLIENT_ID").unwrap_or_else(|_| "admin".to_string());
    let client = dotenv::var("CLIENT").unwrap_or_else(|_| "localhost:8080".to_string());
    let client_secret = dotenv::var("CLIENT_SECRET").unwrap_or_else(|_| "admin".to_string());
    let connection = dotenv::var("CONNECTION")
        .unwrap_or_else(|_| "Username-Password-Authentication".to_string());
    let bind = dotenv::var("BIND").unwrap_or_else(|_| "localhost:8080".to_string());

    let auth0_service = Auth0Service::new(client_id, client_secret, connection, client);

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .configure(configure_routes)
            .configure(configure_data(db.clone(), auth0_service.clone()))
    })
    .bind(bind)?;

    server.run().await?;
    Ok(())
}
