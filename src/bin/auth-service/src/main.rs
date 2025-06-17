pub mod consts;
pub mod errors;
pub mod middleware;
pub mod opts;
pub mod services;
pub mod utils;

use crate::errors::Result;
use crate::opts::app::AppState;
use crate::opts::cmd_opts::{load_configurations, Opts};
use crate::services::auth0::auth0_service::Auth0Service;
use crate::services::db::postgres_db::DbService;
use crate::services::db::utils::create_connection_pool;
use crate::services::routes::configure_routes;
use crate::utils::{configure_data, init_logging};
use actix::Actor;
use jsonwebtoken::DecodingKey;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::services::actix_requests::requests::login,
        crate::services::actix_requests::requests::register,
        crate::services::actix_requests::requests::change_password
    ),
    components(
        schemas(crate::services::actix_requests::models::RegisteredUserData),
        schemas(crate::services::actix_requests::models::UserData),
        schemas(crate::services::actix_requests::models::UpdatePasswordData),
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    init_logging()?;
    log::info!("Starting auth service...");

    let opts = load_configurations()?;

    let bind = opts.application.bind.clone();

    let state = init_state(opts).await?;

    let server = actix_web::HttpServer::new(move || {
        let state = state.clone();

        actix_web::App::new()
            .configure(configure_routes)
            .configure(configure_data(state))
    })
    .bind(bind)?;

    server.run().await?;
    Ok(())
}

async fn init_state(opts: Opts) -> Result<AppState> {
    let pool = create_connection_pool(opts.database.database_url).await?;
    let db = DbService::new(pool);
    let db = db.start();

    let decoding_key = get_secret(&opts.auth0.dev_key_file);

    let auth0 = Auth0Service::new(
        opts.auth0.client_id,
        opts.auth0.client_secret,
        opts.auth0.connection,
        opts.auth0.client,
        opts.auth0.audience,
        decoding_key,
    );

    Ok(AppState::new(db, auth0))
}

fn get_secret(path: &str) -> DecodingKey {
    let pem_bytes = std::fs::read(path).expect("Failed to read PEM file");
    DecodingKey::from_rsa_pem(&pem_bytes).expect("Failed to load key")
}
