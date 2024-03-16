use crate::actors::messages::{CheckIfRegisteredUser, CheckUser, CreateUser};
use crate::db::postgres_db::DbService;
use crate::models::{RegisteredUserData, UserData};
use crate::user_flow::auth0::{get_jwt_user_token, register_user};
use actix::Addr;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use uuid::Uuid;

pub async fn register(
    user: Json<UserData>,
    db: Data<Addr<DbService>>,
) -> crate::errors::Result<HttpResponse> {
    let user_id = Uuid::new_v4();
    register_user(user.clone(), user_id).await?;

    let if_user = CheckIfRegisteredUser {
        username: user.username.clone(),
        email: user.email.clone(),
    };

    if db.send(if_user).await?? {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let user = CreateUser {
        id: user_id,
        username: user.username.clone(),
        password: user.password.clone(),
        email: user.email.clone(),
    };

    db.send(user.clone()).await??;

    log::info!("Getting request for register!");
    Ok(HttpResponse::Ok().body(serde_json::to_string(&user)?))
}

pub async fn login(
    db: Data<Addr<DbService>>,
    user: Json<RegisteredUserData>,
) -> crate::errors::Result<HttpResponse> {
    let if_user = CheckUser { id: user.id };

    if db.send(if_user).await?? {
        log::info!("Getting request for login!");
        let token = get_jwt_user_token(user.0.clone()).await?;
        let json = serde_json::json!({ "user": user, "token": token });
        Ok(HttpResponse::Ok().body(json.to_string()))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

pub async fn check_token(_db: Data<DbService>) -> crate::errors::Result<HttpResponse> {
    log::info!("Getting request for checking token!");
    Ok(HttpResponse::Ok().finish())
}
