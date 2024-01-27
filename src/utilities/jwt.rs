use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    role: String,
}

pub fn create_token(secret: &str, user_id: i32, role: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::days(7);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
        role,
    };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })
        .map(|_claim| true)
}

pub fn validate_admin_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let result = decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })?
        .claims;

    if result.role == "admin" {
        Ok(true)
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not permitted to do this!",
        ))
    }
}
