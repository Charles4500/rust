use axum::{extract::State, Json};
use serde_json::Value;

use crate::{
    common::validation::ValidatedJson,
    models::mpesa::{StkPushRequest, StkPushResponse},
    state::AppState,
};

pub async fn stk_push(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<StkPushRequest>,
) -> Result<Json<StkPushResponse>, (axum::http::StatusCode, String)> {
    let response = state
        .mpesa
        .stk_push(request)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(response))
}

pub async fn callback(State(_state): State<AppState>, Json(body): Json<Value>) -> &'static str {
    println!("{}", serde_json::to_string_pretty(&body).unwrap());

    "OK"
}
