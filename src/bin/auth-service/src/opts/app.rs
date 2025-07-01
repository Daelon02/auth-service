use crate::services::auth0::auth0_service::Auth0Service;
use crate::services::db::postgres_db::DbService;
use actix::Addr;

#[derive(Clone)]
pub struct AppState {
    pub database: Addr<DbService>,
    pub auth0: Auth0Service,
}

impl AppState {
    pub fn new(database: Addr<DbService>, auth0: Auth0Service) -> Self {
        Self { database, auth0 }
    }
}
