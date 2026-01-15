pub mod jwt;
pub mod password;
pub mod error_handler;

pub use jwt::{Claims, User, generate_claims, generate_token_from_claims, decode_jwt};
pub use password::{hash_password, verify_password};
pub use error_handler::CustomError;