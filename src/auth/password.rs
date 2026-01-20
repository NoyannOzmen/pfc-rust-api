use bcrypt::{hash, verify};

use crate::auth::CustomError;

pub fn hash_password(password: &str) -> Result<String, CustomError> {
    Ok(hash(password, 8).unwrap())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, CustomError> {
    Ok(verify(password, &hashed).unwrap())
}