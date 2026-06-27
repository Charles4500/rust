use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::{
    dto::user_dto::{CreateUserDTO, LoginDto},
    models::user::User,
    utils::utils::create_jwt,
};
use crate::{models::user::NewUser, schema::users::dsl::*, state::AppState};
use uuid::Uuid;

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<CreateUserDTO>,
) -> impl IntoResponse {
    let mut conn = match state.db_pool.get().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Pool Error").into_response(),
    };

    // Check existence
    let exists = users
        .filter(email.eq(&body.email))
        .select(id)
        .first::<Uuid>(&mut conn)
        .await
        .optional();

    if matches!(exists, Ok(Some(_))) {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "Email exists"})),
        )
            .into_response();
    }

    // Hash Password
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: &body.email,
        password_hash: Some(hash),
        name: &body.name,
        provider: "local",
    };

    match diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(serde_json::json!({"status": "created"})),
        )
            .into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    }
}

pub async fn login(
    jar: CookieJar,
    State(state): State<AppState>,
    Json(body): Json<LoginDto>,
) -> impl IntoResponse {
    let mut conn = match state.db_pool.get().await {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "DB Pool Error"
                })),
            )
                .into_response();
        }
    };

    let found_user = users
        .filter(email.eq(&body.email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .optional();

    match found_user {
        Ok(Some(user)) if user.provider == "local" => {
            if let Some(hash) = &user.password_hash {
                let parsed = match PasswordHash::new(hash) {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(serde_json::json!({
                                "error": "Invalid password hash"
                            })),
                        )
                            .into_response();
                    }
                };

                if Argon2::default()
                    .verify_password(body.password.as_bytes(), &parsed)
                    .is_ok()
                {
                    let token = create_jwt(&user.id.to_string(), &state.jwt_secret);

                    let cookie = Cookie::build(("auth_token", token))
                        .path("/")
                        .http_only(true)
                        .build();

                    let jar = jar.add(cookie);

                    return (
                        StatusCode::OK,
                        jar,
                        Json(serde_json::json!({
                            "message": "Login successful"
                        })),
                    )
                        .into_response();
                }
            }
        }

        Ok(Some(_)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Use Google Login"
                })),
            )
                .into_response();
        }

        _ => {}
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({
            "error": "Invalid credentials"
        })),
    )
        .into_response()
}
