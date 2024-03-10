use crate::actors::messages::CreateUser;
use crate::db::postgres_db::DbService;
use crate::db::utils::get_jwt_token;
use crate::errors::Result;
use actix::Addr;
use actix_web::web::Data;
use actix_web::HttpResponse;

pub async fn register(db: Data<Addr<DbService>>) -> Result<HttpResponse> {
    let token = get_jwt_token().await?;
    db.send(CreateUser {
        id: Default::default(),
        username: "".to_string(),
        password: "".to_string(),
        email: "".to_string(),
    })
    .await??;
    log::info!("Getting request for register!");
    Ok(HttpResponse::Ok().body(token))
}

pub async fn login(_db: Data<DbService>) -> Result<HttpResponse> {
    log::info!("Getting request for login!");
    Ok(HttpResponse::Ok().finish())
}

pub async fn check_token(_db: Data<DbService>) -> Result<HttpResponse> {
    log::info!("Getting request for checking token!");
    Ok(HttpResponse::Ok().finish())
}
