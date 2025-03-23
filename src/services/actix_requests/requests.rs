use crate::errors::Result;
use crate::services::actix_requests::models::{RegisteredUserData, UpdatePasswordData, UserData};
use crate::services::actors::messages::{CheckUser, CreateUser};
use crate::services::auth0::auth0_service::Auth0Service;
use crate::services::auth0::models::Claims;
use crate::services::db::postgres_db::DbService;
use actix::Addr;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

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
) -> Result<HttpResponse> {
    log::info!("Getting request for register!");
    let auth0_response = auth0_service.register_user(user.clone()).await?;

    let user = CreateUser {
        id: auth0_response._id.clone(),
        username: user.username.clone(),
        password: user.password.clone(),
        email: user.email.clone(),
    };

    db.send(user.clone()).await??;

    Ok(HttpResponse::Ok().body(serde_json::to_string(&auth0_response)?))
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
    auth0_service: Data<Auth0Service>,
    db: Data<Addr<DbService>>,
    user: Json<RegisteredUserData>,
) -> Result<HttpResponse> {
    let if_user = CheckUser {
        id: user.id.clone(),
    };

    if db.send(if_user).await?? {
        log::info!("Getting request for login!");
        let result = auth0_service.send_request_to_login(user.0).await?;
        Ok(HttpResponse::Ok().body(serde_json::to_string(&result)?))
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
) -> Result<HttpResponse> {
    let if_user = CheckUser {
        id: user.user_id.clone(),
    };

    if db.send(if_user).await?? {
        log::info!("Getting request for change password!");

        auth0_service
            .send_request_to_change_pass(user.user_id.clone(), user.email.clone())
            .await?;

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
) -> Result<HttpResponse> {
    log::info!("Getting request for profile!");

    let Some(auth_header) = req.headers().get("Authorization") else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let token = auth_header.to_str()?;

    let access_token = token
        .strip_prefix("Bearer ")
        .ok_or(crate::errors::Error::StringError(
            "Invalid token".to_string(),
        ))?;

    let user_id = extract_user_id(access_token, get_secret())?;

    let if_user = CheckUser { id: user_id };

    if !db_service.send(if_user).await?? {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let profile = auth0_service
        .send_request_to_get_profile(access_token)
        .await?;
    Ok(HttpResponse::Ok().body(profile))
}

fn extract_user_id(token: &str, secret: DecodingKey) -> Result<String> {
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_audience(&["https://someexample.com"]);

    log::info!("Starting to decode token");

    let token_data = decode::<Claims>(token, &secret, &validation)?;

    log::info!("Token: {:?}", token_data.claims);

    let user_id = token_data.claims.sub.trim_start_matches("auth0|");

    Ok(user_id.to_string())
}

fn get_secret() -> DecodingKey {
    let pem_bytes = std::fs::read("dev-gf1e5jqvirizjwfz.pem").expect("Failed to read PEM file");
    DecodingKey::from_rsa_pem(&pem_bytes).expect("Failed to load key")
}
