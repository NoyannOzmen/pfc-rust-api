use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, /* HttpRequest, */ HttpResponse, web};
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::auth::jwt::{/* Claims, User, */ generate_claims, generate_token_from_claims, /* decode_jwt */};
use crate::auth::password::verify_password;
use crate::database::models::UtilisateurModel;
/* use crate::database::models::UtilisateurModel; */
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
    pub user: UtilisateurModel
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(login));
}

async fn login(db: web::Data<DbConn>, req: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    process_json_validation(&req)?;

    let user_repository = UtilisateurRepository::new(db.get_ref());

    let user = match user_repository
        .find_by_email(&req.email)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?
    {
        Some(user) => user,
        None => return Err(ErrorUnauthorized("Account not registered".to_string())),
    };

    let is_valid = verify_password(&req.mot_de_passe, &user.mot_de_passe)?;
    if !is_valid {
        return Err(ErrorUnauthorized("Invalid credentials"));
    }

    let claims = generate_claims(&user);
    let access_token = generate_token_from_claims(&claims)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        access_token,
        user
    }))
}
