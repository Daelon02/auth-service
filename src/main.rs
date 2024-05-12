pub mod actors;
pub mod consts;
pub mod db;
pub mod errors;
pub mod models;
pub mod user_flow;
pub mod utils;

use crate::db::postgres_db::DbService;
use crate::errors::Result;
use crate::user_flow::requests::{check_token, login, register};
use crate::utils::init_logging;
use actix::Actor;
use actix_web::web::Data;
use utoipa_swagger_ui::SwaggerUi;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::user_flow::requests::login,
        crate::user_flow::requests::register
    ),
    components(
        schemas(crate::models::RegisteredUserData),
        schemas(crate::models::UserData)
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
    let openapi = ApiDoc::openapi();
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route("/login", actix_web::web::post().to(login))
            .route("/register", actix_web::web::post().to(register))
            .route("/check_token", actix_web::web::get().to(check_token))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(Data::new(db.clone()))
    })
    .bind("localhost:8080")?
    .run()
    .await?;
    Ok(())
}
