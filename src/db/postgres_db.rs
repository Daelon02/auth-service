use crate::db::utils::DatabasePool;
use actix::Actor;

#[derive(Clone)]
pub struct DbService {
    pub(crate) pool: DatabasePool,
}

impl DbService {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}

impl Actor for DbService {
    type Context = actix::Context<Self>;
}
