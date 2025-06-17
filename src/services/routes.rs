use crate::middleware::auth::AuthMiddleware;
use crate::services::actix_requests::requests::{change_password, login, profile, register};
use crate::ApiDoc;
use actix_web::web;
use actix_web::web::ServiceConfig;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(
        web::scope("/user")
            .wrap(AuthMiddleware)
            .service(web::resource("/change_password").route(web::post().to(change_password)))
            .service(web::resource("/profile").route(web::get().to(profile))),
    )
    .service(
        web::scope("")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi)),
    );
}
