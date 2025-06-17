use crate::services::actors::messages::{
    CheckIfRegisteredUser, CheckUser, CreateUser, DeleteUser, UpdateActivateEmail, UpdateEmail,
    UpdateUsername,
};
use crate::services::db::postgres_db::DbService;
use crate::services::db::tables::Users;
use actix::{AtomicResponse, Handler, WrapFuture};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

impl Handler<CreateUser> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let user_id = msg.id.clone();
        let result = async move {
            let user = Users {
                auth_id: user_id,
                username: msg.username,
                email: msg.email,
                is_email_activate: false,
                created_at: chrono::Utc::now(),
                updated_at: None,
            };

            let _ = diesel::insert_into(crate::services::db::schema::users::table)
                .values(user)
                .execute(&mut conn.await?)
                .await?;
            Ok(())
        };
        log::info!("Creating user {}", msg.id);

        let db = self.clone();

        AtomicResponse::new(Box::pin(result.into_actor(&db)))
    }
}

impl Handler<UpdateActivateEmail> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: UpdateActivateEmail, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };

        let user_id = msg.user_id.clone();
        let query = async move {
            let _ = diesel::update(crate::services::db::schema::users::table)
                .filter(crate::services::db::schema::users::auth_id.eq(user_id))
                .set(crate::services::db::schema::users::is_email_activate.eq(true))
                .execute(&mut conn.await?)
                .await?;
            Ok(())
        };
        log::info!("Updating user is_activate_email {}", msg.user_id);

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}

impl Handler<DeleteUser> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let user_id = msg.user_id.clone();
        let query = async move {
            let _ = diesel::delete(
                crate::services::db::schema::users::table
                    .filter(crate::services::db::schema::users::auth_id.eq(user_id)),
            )
            .execute(&mut conn.await?)
            .await?;
            Ok(())
        };
        log::info!("Deleting user {}", msg.user_id);

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}

impl Handler<UpdateEmail> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: UpdateEmail, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let user_id = msg.user_id.clone();
        let query = async move {
            let _ = diesel::update(crate::services::db::schema::users::table)
                .filter(crate::services::db::schema::users::auth_id.eq(user_id))
                .set(crate::services::db::schema::users::email.eq(msg.email))
                .execute(&mut conn.await?)
                .await?;
            Ok(())
        };
        log::info!("Updating user email {}", msg.user_id);

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}

impl Handler<UpdateUsername> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: UpdateUsername, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let user_id = msg.user_id.clone();
        let query = async move {
            let _ = diesel::update(crate::services::db::schema::users::table)
                .filter(crate::services::db::schema::users::auth_id.eq(user_id))
                .set(crate::services::db::schema::users::username.eq(msg.username))
                .execute(&mut conn.await?)
                .await?;
            Ok(())
        };
        log::info!("Updating user username {}", msg.user_id);

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}

impl Handler<CheckUser> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<bool>>;

    fn handle(&mut self, msg: CheckUser, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let user_id = msg.id.clone();
        let query = async move {
            let user = crate::services::db::schema::users::table
                .count()
                .filter(crate::services::db::schema::users::auth_id.eq(user_id.clone()))
                .first::<i64>(&mut conn.await?)
                .await?;
            Ok(user == 1)
        };
        log::info!("Checking user {}", msg.id);

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}

impl Handler<CheckIfRegisteredUser> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<bool>>;

    fn handle(&mut self, msg: CheckIfRegisteredUser, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let query = async move {
            let user = crate::services::db::schema::users::table
                .count()
                .filter(crate::services::db::schema::users::username.eq(msg.username))
                .filter(crate::services::db::schema::users::email.eq(msg.email))
                .first::<i64>(&mut conn.await?)
                .await?;

            Ok(user == 1)
        };
        log::info!("Checking if user is registered");

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}
