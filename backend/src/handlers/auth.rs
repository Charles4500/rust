use axum::response::IntoResponse;
use oauth2_passkey_axum::AuthUser;

pub async fn me(user: AuthUser) -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "id": user.id,
        "account": user.account,
    }))
}
