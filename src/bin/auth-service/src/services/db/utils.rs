use crate::errors::Result;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DatabasePool = Pool<AsyncPgConnection>;

pub async fn create_connection_pool(database_url: String) -> Result<DatabasePool> {
    let manager = AsyncDieselConnectionManager::new(database_url);
    let pool = Pool::builder(manager).build()?;

    Ok(pool)
}
