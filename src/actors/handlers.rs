use crate::actors::messages::{
    CheckIfRegisteredUser, CheckUser, CreateUser, DeleteUser, UpdateActivateEmail, UpdateEmail,
    UpdatePassword, UpdateUsername,
};
use crate::db::postgres_db::DbService;
use crate::db::tables::Users;
use actix::{AtomicResponse, Handler, WrapFuture};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

impl Handler<CreateUser> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let result = async move {
            let user = Users {
                id: msg.id,
                username: msg.username,
                email: msg.email,
                password: msg.password,
                is_email_activate: false,
                created_at: chrono::Utc::now(),
                updated_at: None,
            };

            let _ = diesel::insert_into(crate::db::schema::users::table)
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

        let query = async move {
            let _ = diesel::update(crate::db::schema::users::table)
                .filter(crate::db::schema::users::id.eq(msg.user_id))
                .set(crate::db::schema::users::is_email_activate.eq(true))
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
        let query = async move {
            let _ = diesel::delete(
                crate::db::schema::users::table
                    .filter(crate::db::schema::users::id.eq(msg.user_id)),
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

impl Handler<UpdatePassword> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: UpdatePassword, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let query = async move {
            let _ = diesel::update(crate::db::schema::users::table)
                .filter(crate::db::schema::users::id.eq(msg.user_id))
                .set(crate::db::schema::users::password.eq(msg.password))
                .execute(&mut conn.await?)
                .await?;
            Ok(())
        };
        log::info!("Updating user password {}", msg.user_id);

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}

impl Handler<UpdateEmail> for DbService {
    type Result = AtomicResponse<Self, crate::errors::Result<()>>;

    fn handle(&mut self, msg: UpdateEmail, _: &mut Self::Context) -> Self::Result {
        let db = self.clone();
        let conn = async move { db.pool.get().await };
        let query = async move {
            let _ = diesel::update(crate::db::schema::users::table)
                .filter(crate::db::schema::users::id.eq(msg.user_id))
                .set(crate::db::schema::users::email.eq(msg.email))
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
        let query = async move {
            let _ = diesel::update(crate::db::schema::users::table)
                .filter(crate::db::schema::users::id.eq(msg.user_id))
                .set(crate::db::schema::users::username.eq(msg.username))
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
        let query = async move {
            let user = crate::db::schema::users::table
                .filter(crate::db::schema::users::id.eq(msg.id))
                .first::<Users>(&mut conn.await?)
                .await?;
            Ok(user.id == msg.id)
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
            let user = crate::db::schema::users::table
                .filter(crate::db::schema::users::username.eq(msg.username))
                .filter(crate::db::schema::users::email.eq(msg.email))
                .first::<Users>(&mut conn.await?)
                .await;

            match user {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        };
        log::info!("Checking if user is registered");

        let db = self.clone();
        AtomicResponse::new(Box::pin(query.into_actor(&db)))
    }
}
