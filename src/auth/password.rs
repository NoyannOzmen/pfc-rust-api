use actix_web::{Error};
use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> Result<String, Error> {
    Ok(hash(password, 8).unwrap())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, Error> {
    Ok(verify(password, &hashed).unwrap())
}