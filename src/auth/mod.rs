pub mod jwt;
pub mod password;
pub mod error_handler;

pub use jwt::{Claims, User, generate_claims, generate_token_from_claims, decode_jwt, extract_user_id_from_token, /* extract_foster_id_from_token, extract_shelter_id_from_token */};
pub use password::{hash_password, verify_password};
pub use error_handler::CustomError;