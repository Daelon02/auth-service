use crate::actors::messages::CreateUser;
use crate::db::postgres_db::DbService;
use crate::db::utils::{get_jwt_user_token, register_user};
use crate::errors::Result;
use crate::models::UserData;
use actix::Addr;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn register(user: Json<UserData>, db: Data<Addr<DbService>>) -> Result<HttpResponse> {
    let user_id = Uuid::new_v4();
    register_user(user.clone(), user_id).await?;

    let token = get_jwt_user_token(user.clone()).await?;

    db.send(CreateUser {
        id: user_id,
        username: user.username.clone(),
        password: user.password.clone(),
        email: user.email.clone(),
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
