use crate::consts::{AUDIENCE, GRANT_TYPE_PASS};
use crate::user_flow::auth0_models::{ChangePassFlow, ConnectToAuth0, LoginFlow, RegistrationFlow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth0Service {
    pub client_id: String,
    pub client_secret: String,
    pub connection: String,
    pub client_url: String,
}

impl Auth0Service {
    pub fn new(
        client_id: String,
        client_secret: String,
        connection: String,
        client_url: String,
    ) -> Self {
        Auth0Service {
            client_id,
            client_secret,
            connection,
            client_url,
        }
    }

    pub fn generate_body_for_registration(
        &self,
        user_id: Uuid,
        username: String,
        password: String,
        email: String,
    ) -> ConnectToAuth0<RegistrationFlow> {
        ConnectToAuth0 {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            audience: AUDIENCE.to_string(),
            grant_type: GRANT_TYPE_PASS.to_string(),
            user_id: user_id.to_string(),
            connection: self.connection.clone(),
            extra: RegistrationFlow {
                username,
                password,
                email,
            },
        }
    }

    pub async fn generate_body_for_login(
        &self,
        user_id: Uuid,
        username: String,
        password: String,
    ) -> ConnectToAuth0<LoginFlow> {
        ConnectToAuth0 {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            audience: AUDIENCE.to_string(),
            grant_type: GRANT_TYPE_PASS.to_string(),
            user_id: user_id.to_string(),
            connection: self.connection.clone(),
            extra: LoginFlow { username, password },
        }
    }

    pub async fn generate_body_for_change_password(
        &self,
        user_id: Uuid,
        email: String,
    ) -> ConnectToAuth0<ChangePassFlow> {
        ConnectToAuth0 {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            audience: AUDIENCE.to_string(),
            grant_type: GRANT_TYPE_PASS.to_string(),
            user_id: user_id.to_string(),
            connection: self.connection.clone(),
            extra: ChangePassFlow { email },
        }
    }
}
