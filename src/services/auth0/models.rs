use crate::errors::Result;
use crate::services::auth0::errors::BuildError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Default)]
pub struct Auth0RequestBuilder<T: Serialize + DeserializeOwned + Debug> {
    client_id: Option<String>,
    client_secret: Option<String>,
    audience: Option<String>,
    grant_type: Option<String>,
    user_id: Option<String>,
    connection: Option<String>,
    scope: Option<String>,
    extra: Option<T>,
}

impl<T: Serialize + DeserializeOwned + Debug + Default> Auth0RequestBuilder<T> {
    pub fn new() -> Self {
        Auth0RequestBuilder::default()
    }

    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }

    pub fn audience(mut self, audience: String) -> Self {
        self.audience = Some(audience);
        self
    }

    pub fn grant_type(mut self, grant_type: String) -> Self {
        self.grant_type = Some(grant_type);
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn connection(mut self, connection: String) -> Self {
        self.connection = Some(connection);
        self
    }

    pub fn extra(mut self, extra: T) -> Self {
        self.extra = Some(extra);
        self
    }

    pub fn scope(mut self, scope: String) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn build(self) -> Result<ConnectToAuth0<T>> {
        Ok(ConnectToAuth0 {
            client_id: self.client_id.ok_or(BuildError::MissingClientId)?,
            client_secret: self.client_secret.ok_or(BuildError::MissingClientSecret)?,
            audience: self.audience.ok_or(BuildError::MissingAudience)?,
            grant_type: self.grant_type.ok_or(BuildError::MissingGrantType)?,
            user_id: self.user_id.ok_or(BuildError::MissingUserId)?,
            connection: self.connection.ok_or(BuildError::MissingConnection)?,
            extra: self.extra.ok_or(BuildError::MissingExtra)?,
            scope: self.scope,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignupRequestBuilder {
    pub client_id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub connection: Option<String>,
}

impl SignupRequestBuilder {
    pub fn new() -> Self {
        SignupRequestBuilder::default()
    }

    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn connection(mut self, connection: String) -> Self {
        self.connection = Some(connection);
        self
    }
    pub fn build(self) -> Result<SignupRequest> {
        Ok(SignupRequest {
            client_id: self.client_id.ok_or(BuildError::MissingClientId)?,
            username: self.username.ok_or(BuildError::MissingUsername)?,
            password: self.password.ok_or(BuildError::MissingPassword)?,
            email: self.email.ok_or(BuildError::MissingEmail)?,
            connection: self.connection.ok_or(BuildError::MissingConnection)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignupRequest {
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub connection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignInRequestBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    audience: Option<String>,
    grant_type: Option<String>,
}

impl SignInRequestBuilder {
    pub fn new() -> Self {
        SignInRequestBuilder::default()
    }

    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }

    pub fn audience(mut self, audience: String) -> Self {
        self.audience = Some(audience);
        self
    }

    pub fn grant_type(mut self, grant_type: String) -> Self {
        self.grant_type = Some(grant_type);
        self
    }

    pub fn build(self) -> Result<SignInRequest> {
        Ok(SignInRequest {
            client_id: self.client_id.ok_or(BuildError::MissingClientId)?,
            client_secret: self.client_secret.ok_or(BuildError::MissingClientSecret)?,
            audience: self.audience.ok_or(BuildError::MissingAudience)?,
            grant_type: self.grant_type.ok_or(BuildError::MissingGrantType)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignInRequest {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
    pub grant_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectToAuth0<T: Serialize> {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
    pub grant_type: String,
    pub user_id: String,
    pub connection: String,
    pub scope: Option<String>,
    #[serde(flatten)]
    pub extra: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistrationFlow {
    pub client_id: String,
    pub client_secret: String,
    pub password: String,
    pub email: String,
    pub connection: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginFlow {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangePassFlow {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth0RegisterResponse {
    pub _id: Box<str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisteredUserResponse {
    pub id: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct Auth0LoginResponse {
    pub access_token: String,
    pub id_token: String,
    pub expires_in: u64,
    pub token_type: String,
}
