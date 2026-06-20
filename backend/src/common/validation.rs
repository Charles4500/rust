use axum::{
    extract::{FromRequest, Json, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug)]
pub enum ValidatedJsonError {
    InvalidJson(String),
    ValidationFailed(String),
}

impl IntoResponse for ValidatedJsonError {
    fn into_response(self) -> Response {
        match self {
            ValidatedJsonError::InvalidJson(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            ValidatedJsonError::ValidationFailed(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
            }
        }
    }
}

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidatedJsonError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract JSON
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| ValidatedJsonError::InvalidJson(err.to_string()))?;

        // Validate
        value.validate().map_err(|errors| {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .map(|(field, field_errors)| {
                    let messages: Vec<String> = field_errors
                        .iter()
                        .filter_map(|e| e.message.as_ref().map(|msg| msg.to_string()))
                        .collect();
                    format!("{}: {}", field, messages.join(", "))
                })
                .collect();
            ValidatedJsonError::ValidationFailed(error_messages.join("; "))
        })?;

        Ok(ValidatedJson(value))
    }
}
