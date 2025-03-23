use crate::services::auth0::models::Claims;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::RwLock;

pub const AUDIENCE: &str = "https://someexample.com";

pub struct AuthMiddleware;

pub struct CheckAuthMiddleware<S> {
    service: Rc<S>,
    decoding_key: Arc<RwLock<DecodingKey>>,
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = CheckAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        let decoding_key = AuthMiddleware::new_from_file(&jwt_secret).expect("Failed to load key");
        ok(CheckAuthMiddleware {
            service: Rc::new(service),
            decoding_key: Arc::new(RwLock::new(decoding_key)),
        })
    }
}

impl<S> Service<ServiceRequest> for CheckAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let decoding_key = self.decoding_key.clone();

        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");

            if req.path() == "/login" || req.path() == "/register" || req.path() == "/" {
                return Ok(req.into_response(HttpResponse::Ok().finish()));
            }

            if let Some(auth_header) = auth_header {
                if let Ok(auth_token) = auth_header.to_str() {
                    if let Some(auth_str) = auth_token.strip_prefix("Bearer ") {
                        let token = auth_str;

                        let decoding_key = decoding_key.read().await;

                        let audience = [AUDIENCE];

                        let validation = &mut Validation::new(Algorithm::RS256);

                        validation.set_audience(&audience);

                        return match decode::<Claims>(token, &decoding_key, validation) {
                            Ok(_) => service.call(req).await,
                            Err(e) => {
                                log::error!("Unauthorized access: {}", e);
                                Ok(req.into_response(HttpResponse::Unauthorized().finish()))
                            }
                        };
                    }
                }
            }

            Ok(req.into_response(HttpResponse::Unauthorized().finish()))
        })
    }
}

impl AuthMiddleware {
    pub fn new_from_file(path: &str) -> Result<DecodingKey, std::io::Error> {
        let mut file = File::open(path)?;
        let mut key_data = Vec::new();
        file.read_to_end(&mut key_data)?;

        let decoding_key =
            DecodingKey::from_rsa_pem(key_data.as_slice()).expect("Failed to load key");

        Ok(decoding_key)
    }
}
