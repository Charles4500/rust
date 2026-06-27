use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::user::Claims;

pub fn create_jwt(user_id: &str, secret: &str) -> String {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 3600, // 1 hour
        iat: now,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
