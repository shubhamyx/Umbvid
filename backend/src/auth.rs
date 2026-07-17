use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;
use crate::AppState;

const TOKEN_LIFETIME_HOURS: i64 = 24 * 7; // 1 week

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user id
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Auth(format!("failed to hash password: {e}")))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::Auth(format!("failed to verify password: {e}")))
}

pub fn create_jwt(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let exp = (chrono::Utc::now() + chrono::Duration::hours(TOKEN_LIFETIME_HOURS)).timestamp() as usize;
    let claims = Claims { sub: user_id, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Auth(format!("failed to create token: {e}")))
}

fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Auth("invalid or expired token".to_string()))
}

/// Axum extractor: add `auth_user: AuthUser` as a handler argument to require
/// a valid `Authorization: Bearer <token>` header. Gives you the caller's user_id.
/// 


pub struct AuthUser {
    pub user_id: Uuid,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Auth("missing or malformed Authorization header".to_string()))?;

        let app_state = State::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Auth("failed to load app state".to_string()))?
            .0;

        let claims = verify_jwt(bearer.token(), &app_state.jwt_secret)?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}