use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_date: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
