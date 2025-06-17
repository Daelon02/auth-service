use crate::consts::{ACCESS_TOKEN, APPLICATION_JSON, AUTHORIZATION, CONTENT_TYPE, GRANT_TYPE_PASS};
use crate::errors::{Error, Result};
use crate::services::actix_requests::models::{LoginUserResponse, RegisteredUserData, UserData};
use crate::services::auth0::consts::{
    CHANGE_PASSWORD_URL, GET_PROFILE_URL, LOGIN_URL, REGISTRATION_URL, SCOPE,
};
use crate::services::auth0::models::{
    Auth0LoginResponse, Auth0RegisterResponse, Auth0RequestBuilder, ChangePassFlow, Claims,
    ConnectToAuth0, LoginFlow, SignupRequest, SignupRequestBuilder,
};
use http::Method;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Auth0Service {
    client_id: Box<str>,
    client_secret: String,
    connection: String,
    client_url: String,
    audience: String,
    decoding_key: DecodingKey,
}

impl Auth0Service {
    pub fn new(
        client_id: Box<str>,
        client_secret: String,
        connection: String,
        client_url: String,
        audience: String,
        decoding_key: DecodingKey,
    ) -> Self {
        Auth0Service {
            client_id,
            client_secret,
            connection,
            client_url,
            audience,
            decoding_key,
        }
    }

    fn build_base_request<T: Serialize + DeserializeOwned + Debug + Default>(
        &self,
        body: Auth0RequestBuilder<T>,
    ) -> Auth0RequestBuilder<T> {
        body.client_id(self.client_id.to_string())
            .client_secret(self.client_secret.clone())
            .audience(self.audience.to_string())
            .connection(self.connection.clone())
    }

    fn build_body_for_change_password(
        &self,
        user_id: String,
        email: String,
    ) -> Result<ConnectToAuth0<ChangePassFlow>> {
        let body = Auth0RequestBuilder::new();

        let body = self
            .build_base_request(body)
            .grant_type(GRANT_TYPE_PASS.to_string())
            .user_id(user_id)
            .extra(ChangePassFlow { email });

        body.build()
    }

    fn build_body_for_login(&self, user: RegisteredUserData) -> Result<ConnectToAuth0<LoginFlow>> {
        let body = Auth0RequestBuilder::new();

        let body = self
            .build_base_request(body)
            .grant_type(GRANT_TYPE_PASS.to_string())
            .user_id(user.id)
            .scope(SCOPE.to_string())
            .extra(LoginFlow {
                username: user.username,
                password: user.password,
            });

        body.build()
    }

    fn build_body_for_register(
        &self,
        password: Box<str>,
        email: Box<str>,
        username: Box<str>,
    ) -> Result<SignupRequest> {
        let body = SignupRequestBuilder::new();

        let body = body
            .client_id(self.client_id.to_string())
            .email(email.to_string())
            .password(password.to_string())
            .connection(self.connection.clone())
            .username(username.to_string());

        body.build()
    }

    pub async fn send_request_to_change_pass(&self, user_id: String, email: String) -> Result<()> {
        let client = Client::new();

        let url = format!("{}/{}", self.client_url, CHANGE_PASSWORD_URL);

        let body = self.build_body_for_change_password(user_id, email)?;

        client
            .request(Method::POST, &url)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn register_user(&self, user: UserData) -> Result<Auth0RegisterResponse> {
        let client = Client::new();

        let url = format!("{}/{}", self.client_url, REGISTRATION_URL);

        let body = self.build_body_for_register(user.password, user.email, user.username)?;

        let result = client.post(&url).json(&body).send().await?;

        if result.status().is_success() {
            let result = result.json::<Value>().await?;
            let result = serde_json::from_value::<Auth0RegisterResponse>(result)?;
            Ok(result)
        } else {
            Err(Error::StringError(result.text().await?))
        }
    }

    pub async fn get_jwt_user_token(&self, user: RegisteredUserData) -> Result<String> {
        let client = Client::new();

        let url = format!("{}/{}", self.client_url, LOGIN_URL);

        let body = self.build_body_for_login(user)?;

        let response = client
            .request(Method::POST, &url)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .json(&body)
            .send()
            .await?;

        let response: Result<Auth0LoginResponse> = response.json().await.map_err(Error::from);
        match response {
            Ok(value) => Ok(value.access_token),
            Err(e) => {
                log::error!("Error: {}", e);
                Err(Error::InvalidToken)
            }
        }
    }

    pub async fn send_request_to_get_profile(&self, access_token: &str) -> Result<String> {
        let client = Client::new();

        let url = format!("{}/{}", self.client_url, GET_PROFILE_URL);
        log::info!("Requesting profile from URL: {}", &url);

        let result = client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .send()
            .await;

        match result {
            Ok(response) => {
                log::info!("Response status: {}", response.status());
                if response.status().is_success() {
                    let body = response.text().await?;
                    log::info!("Response body: {:?}", body);
                    Ok(body)
                } else {
                    let error_body = response.text().await?;
                    log::error!("Error response body: {:?}", error_body);
                    Err(Error::StringError(error_body))
                }
            }
            Err(e) => {
                log::error!("Request failed: {:?}", e);
                Err(Error::StringError(e.to_string()))
            }
        }
    }

    pub async fn send_request_to_login(
        &self,
        user: RegisteredUserData,
    ) -> Result<LoginUserResponse> {
        let client = Client::new();

        let url = format!("{}/{}", self.client_url, LOGIN_URL);

        let body = self.build_body_for_login(user)?;

        let response = client
            .request(Method::POST, &url)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .json(&body)
            .send()
            .await?;

        let response: Result<Value> = response.json().await.map_err(Error::from);
        match response {
            Ok(value) => {
                log::info!("Response: {:?}", value);
                let response = LoginUserResponse {
                    token: value[ACCESS_TOKEN]
                        .as_str()
                        .ok_or(Error::InvalidToken)?
                        .to_string(),
                };
                Ok(response)
            }
            Err(e) => {
                log::error!("Error: {}", e);
                Err(Error::InvalidToken)
            }
        }
    }

    pub fn decoding_key(&self) -> &DecodingKey {
        &self.decoding_key
    }

    pub fn audience(&self) -> &str {
        &self.audience
    }

    pub fn extract_user_id(&self, token: &str) -> Result<String> {
        let mut validation = Validation::new(Algorithm::RS256);

        validation.set_audience(&[&self.audience]);

        log::info!("Starting to decode token");

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)?;

        log::info!("Token: {:?}", token_data.claims);

        let user_id = token_data.claims.sub.trim_start_matches("auth0|");

        Ok(user_id.to_string())
    }
}
