use crate::db::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_email_activate: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
