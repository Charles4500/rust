use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password_hash: Option<String>,
    pub provider: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub password_hash: Option<String>,
    pub name: &'a str,
    pub provider: &'a str,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
