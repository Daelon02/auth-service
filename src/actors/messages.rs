use actix::Message;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct CreateUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdateActivateEmail {
    pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct DeleteUser {
    pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdatePassword {
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdateEmail {
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdateUsername {
    pub user_id: Uuid,
    pub username: String,
}
