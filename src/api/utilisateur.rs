use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnprocessableEntity};
use actix_web::{Error, HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::auth::hash_password;
use crate::database::models::UtilisateurActiveModel;
use crate::database::repositories::UtilisateurRepository;
use crate::validators::common_validators::{process_json_validation};

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").post(create_user)
        );
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_users)
        )
        .service(web::resource("/{id}")
            .get(get_user)
            .put(update_user)
            .delete(delete_user)
        );
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserCreate {
    #[validate(length(
        min = 8,
        max = 30,
        message = "Password must be between 8 and 30 characters"
    ))]
    pub mot_de_passe: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserUpdate {
    #[validate(length(
        min = 8,
        max = 30,
        message = "Password must be between 8 and 30 characters"
    ))]
    pub mot_de_passe: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

pub async fn get_users(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = UtilisateurRepository::new(db.get_ref());

    let users = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve users: {}", e)))?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let repo = UtilisateurRepository::new(db.get_ref());

    let user = repo
        .find_by_id(user_id)
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve user: {}", e)))?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(ErrorNotFound(format!("User with ID {} not found", user_id))),
    }
}

pub async fn create_user(
    db: web::Data<DbConn>,
    json_user: web::Json<UserCreate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_user)?;

    info!(
        "Attempting to create user with email: {}",
        json_user.email
    );

    let repo = UtilisateurRepository::new(db.get_ref());

    if let Some(_) = repo
        .find_by_email(&json_user.email)
        .await
        .map_err(|e| ErrorInternalServerError(e))?
    {
        return Err(ErrorUnprocessableEntity(format!(
            "Something went wrong creating user",
        )));
    }

    let hashed_password = hash_password(&json_user.mot_de_passe)?;

    let user = json_user.into_inner();

    let user_model = UtilisateurActiveModel {
        email: Set(user.email),
        mot_de_passe: Set(hashed_password),
        ..Default::default()
    };

    let created_user = repo
        .create(user_model)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to create user: {}", e)))?;

    info!("User created with ID: {}", created_user.id);
    Ok(HttpResponse::Created().json(created_user))
}

pub async fn update_user(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_user: web::Json<UserUpdate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_user)?;

    let user_id = path.into_inner();

    info!("Attempting to update user with ID: {}", user_id);

    let repo = UtilisateurRepository::new(db.get_ref());

    if let Some(ref email) = json_user.email {
        if email.trim().is_empty() {
            return Err(ErrorUnprocessableEntity("Email cannot be empty"));
        }

        if let Some(existing_user) = repo
            .find_by_email(email)
            .await
            .map_err(|e| ErrorInternalServerError(e))?
        {
            if existing_user.id != user_id {
                return Err(ErrorUnprocessableEntity(format!(
                    "Cannot update user with provided credentials"
                )));
            }
        }
    }

    let user_data = repo
        .find_by_id(user_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    match user_data {
        Some(user_data) => {
            let mut user_active_model: UtilisateurActiveModel = user_data.into();

            let user = json_user.into_inner();

            if let Some(email) = user.email {
                user_active_model.email = Set(email);
            }

            let updated_user = repo
                .update(user_active_model)
                .await
                .map_err(|e| ErrorInternalServerError(format!("Failed to update user: {}", e)))?;

            info!("User with ID {} updated", user_id);
            Ok(HttpResponse::Ok().json(updated_user))
        }
        None => Err(ErrorNotFound(format!("User with ID {} not found", user_id))),
    }
}

pub async fn delete_user(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let repo = UtilisateurRepository::new(db.get_ref());

    info!("Attempting to delete user with ID: {}", user_id);

    let user = repo
        .find_by_id(user_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?;
    if user.is_none() {
        return Err(ErrorNotFound(format!("User with ID {} not found", user_id)));
    }

    let delete_result = repo
        .delete(user_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to delete user: {}", e)))?;

    if delete_result.rows_affected > 0 {
        info!("User with ID {} successfully deleted", user_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("User with ID {} was not deleted (0 rows affected)", user_id);
        Err(ErrorInternalServerError(
            "Failed to delete user (0 rows affected)",
        ))
    }
}