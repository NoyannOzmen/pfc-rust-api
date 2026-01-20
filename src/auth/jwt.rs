use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
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
    pub role: String,
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

    let mut role: String = format!(""); 
    if !user.refuge.is_none() {
        role = format!("SHELTER");
    };
    if !user.accueillant.is_none() {
        role = format!("FOSTER");
    };

    Claims {
        sub: user.id.to_string(),
        exp: expiration,
        iat,
        jti,
        user_id: user.id,
        email: user.email.clone(),
        role: role,
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

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, CustomError> {
  let token_data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
    &Validation::default(),
  ).map_err(|_e| {
        log::error!("JWT validation error: {}", _e);
        CustomError::BadClientData
    })?;

  Ok(token_data)
}

pub fn extract_user_id_from_token(token: &str) -> Option<i32> {
    let validation = Validation::default();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &validation,
    ) {
        Ok(token_data) => Some(token_data.claims.user_id),
        Err(e) => {
            log::error!("Failed to decode token: {:?}", e);
            None
        }
    }
}