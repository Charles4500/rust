use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;

use crate::db::Pool;
use crate::models::user::{NewUser, UpdateUser, User};
use crate::schema::users;

use super::error::internal_error;

pub async fn create_user(
    State(pool): State<Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(user))
}

pub async fn list_users(State(pool): State<Pool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let users = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(users))
}

pub async fn get_user(
    State(pool): State<Pool>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let user = users::table
        .find(user_id)
        .select(User::as_select())
        .first(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                (StatusCode::NOT_FOUND, "User not found".to_string())
            }
            _ => internal_error(e),
        })?;

    Ok(Json(user))
}

pub async fn update_user(
    State(pool): State<Pool>,
    Path(user_id): Path<i32>,
    Json(update_data): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let user = diesel::update(users::table.find(user_id))
        .set(&update_data)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                (StatusCode::NOT_FOUND, "User not found".to_string())
            }
            _ => internal_error(e),
        })?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(pool): State<Pool>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let row_deleted = diesel::delete(users::table.find(user_id))
        .execute(&mut conn)
        .await
        .map_err(internal_error)?;

    if row_deleted == 0 {
        Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
