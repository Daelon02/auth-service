use crate::services::db::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct Users {
    pub auth_id: String,
    pub username: String,
    pub email: String,
    pub is_email_activate: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
