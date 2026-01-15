/* use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized}; */
use actix_web::{HttpResponse, web};
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::auth::CustomError;
use crate::auth::jwt::{generate_claims, generate_token_from_claims};
use crate::auth::password::verify_password;
use crate::database::models::UtilisateurModelEx;
use crate::database::repositories::UtilisateurRepository;

use crate::validators::common_validators::process_json_validation;


#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub mot_de_passe: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub user: UtilisateurModelEx
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(login));
}

async fn login(db: web::Data<DbConn>, req: web::Json<LoginRequest>) -> Result<HttpResponse, CustomError> {
    process_json_validation(&req)?;

    let user_repository = UtilisateurRepository::new(db.get_ref());

    let user = match user_repository
        .find_by_email(&req.email)
        .await
        .map_err(|_e| CustomError::InternalError)?
    {
        Some(user) => user,
        None => return Err(CustomError::WrongLogin),
    };

    let is_valid = verify_password(&req.mot_de_passe, &user.mot_de_passe)?;
    if !is_valid {
        return Err(CustomError::WrongLogin);
    }

    let claims = generate_claims(&user);
    let access_token = generate_token_from_claims(&claims)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        access_token,
        user
    }))
}
