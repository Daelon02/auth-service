use actix::Message;
use serde::Serialize;

#[derive(Message, Serialize, Clone)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct CreateUser {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdateActivateEmail {
    pub user_id: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct DeleteUser {
    pub user_id: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdateEmail {
    pub user_id: String,
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<()>")]
pub(crate) struct UpdateUsername {
    pub user_id: String,
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<bool>")]
pub(crate) struct CheckUser {
    pub id: String,
}

#[derive(Message)]
#[rtype(result = "crate::errors::Result<bool>")]
pub(crate) struct CheckIfRegisteredUser {
    pub username: String,
    pub email: String,
}
