/* use actix_web::error::ErrorInternalServerError;
use actix_web::{Error}; */
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::CustomError;
use crate::database::models::UtilisateurModelEx;

#[derive(Deserialize, Serialize)]
pub struct User {
  email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub user_id: i32,
    pub email: String,
}

pub static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_jwt_secret_for_development_only".into())
});

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_claims(user: &UtilisateurModelEx) -> Claims {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let iat = Utc::now().timestamp() as usize;
    let jti = generate_uuid();

    Claims {
        sub: user.id.to_string(),
        exp: expiration,
        iat,
        jti,
        user_id: user.id,
        email: user.email.clone(),
    }
}

pub fn generate_token_from_claims(claims: &Claims) -> Result<String, CustomError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_e| {log::error!("Error generating token: {}", _e); CustomError::InternalError})
}

pub fn decode_jwt(token: &str) -> Result<User, String> {
  let token_data = decode::<User>(
    token,
    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
    &Validation::default(),
  );

  match token_data {
    Ok(token_data) => Ok(token_data.claims),

    Err(e) => Err(e.to_string()),
  }
}