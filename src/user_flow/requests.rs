use crate::actors::messages::{CheckIfRegisteredUser, CheckUser, CreateUser};
use crate::db::postgres_db::DbService;
use crate::models::{RegisteredUserData, UpdatePasswordData, UserData};
use crate::services::auth0::Auth0Service;
use crate::user_flow::account_flow_methods::{
    send_request_to_change_pass, send_request_to_get_profile,
};
use crate::user_flow::auth0::{get_jwt_user_token, register_user};
use actix::Addr;
use actix_web::web::{Data, Json};
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/register",
    responses(
        (status = 200, description = "User successfully registered", body = UserData),
        (status = BAD_REQUEST, description = "User already registered")
    )
)]
pub async fn register(
    user: Json<UserData>,
    auth0_service: Data<Auth0Service>,
    db: Data<Addr<DbService>>,
) -> crate::errors::Result<HttpResponse> {
    let user_id = Uuid::new_v4();
    register_user(user.clone(), user_id, auth0_service).await?;

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

#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = 200, description = "User successfully login", body = RegisteredUserData),
        (status = BAD_REQUEST, description = "User not found")
    )
)]
pub async fn login(
    user: Json<RegisteredUserData>,
    auth0_service: Data<Auth0Service>,
    db: Data<Addr<DbService>>,
) -> crate::errors::Result<HttpResponse> {
    let if_user = CheckUser { id: user.id };

    if db.send(if_user).await?? {
        log::info!("Getting request for login!");
        let token = get_jwt_user_token(user.0.clone(), auth0_service).await?;
        let json = serde_json::json!({ "user": user, "token": token });
        Ok(HttpResponse::Ok().body(json.to_string()))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

#[utoipa::path(
    post,
    path = "/change_password",
    responses(
        (status = 200, description = "Successfully send email to change password"),
        (status = BAD_REQUEST, description = "User not found")
    )
)]
pub async fn change_password(
    db: Data<Addr<DbService>>,
    user: Json<UpdatePasswordData>,
    auth0_service: Data<Auth0Service>,
) -> crate::errors::Result<HttpResponse> {
    let if_user = CheckUser { id: user.user_id };

    if db.send(if_user).await?? {
        log::info!("Getting request for change password!");

        send_request_to_change_pass(user.user_id, user.email.clone(), auth0_service).await?;

        Ok(HttpResponse::Ok().body("Sent email to change password!"))
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

#[utoipa::path(
    get,
    path = "/profile/{user_id}",
    responses(
        (status = 200, description = "Successfully get user profile", body = String),
        (status = BAD_REQUEST, description = "User not found")
    )
)]
pub async fn profile(
    auth0_service: Data<Auth0Service>,
    db_service: Data<Addr<DbService>>,
    req: HttpRequest,
    user_id: web::Path<Uuid>,
) -> crate::errors::Result<HttpResponse> {
    log::info!("Getting request for profile!");

    let if_user = CheckUser {
        id: user_id.into_inner(),
    };

    if !db_service.send(if_user).await?? {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let access_token = req
        .headers()
        .get("Authorization")
        .expect("No Authorization header")
        .to_str()
        .expect("Invalid Authorization header");

    let profile = send_request_to_get_profile(access_token, auth0_service).await?;
    Ok(HttpResponse::Ok().body(profile))
}
