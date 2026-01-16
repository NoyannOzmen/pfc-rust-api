/* use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnprocessableEntity}; */
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;
use validator::Validate;

use serde::{Deserialize, Serialize};

use crate::auth::{CustomError, hash_password};
use crate::database::models::{FamilleActiveModel, FamilleActiveModelEx, UtilisateurActiveModel};
use crate::database::repositories::{FamilleRepository, UtilisateurRepository};
use crate::validators::common_validators::{process_json_validation, validate_phone, validate_zipcode};

use sea_orm::ActiveValue::Set;

pub fn configure_register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .post(create_foster)
        );
}

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            /* .get(get_fosters) */
            .post(update_foster)
        )
        .service(web::resource("/delete")
            .post(delete_foster)
        )
        .service(web::resource("/{id}")
            .get(get_foster)
        );
}

#[derive(Deserialize, Serialize, Validate)]
pub struct FosterCreate {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your first name must usually be between 2 and 50 characters"
    ))]
    pub prenom: Option<String>,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your full last name should be between 2 and 50 characters"
    ))]
    pub nom: String,
    #[validate(custom(function = validate_phone))]
    pub telephone: String,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your address must usually be between 2 and 50 characters"
    ))]
    pub rue: String,
    #[validate(length(
        min = 2,
        max = 58,
        message = "Your city's name must be between 2 and 58 characters"
    ))]
    pub commune: String,
    #[validate(custom(function = validate_zipcode))]
    pub code_postal: String,
    #[validate(length(
        min = 4,
        max = 42,
        message = "Country name must be between 4 and 42 characters"
    ))]
    pub pays: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe your home using between 3 and 50 characters"
    ))]
    pub hebergement: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe your garden/yard using between 3 and 50 characters"
    ))]
    pub terrain: Option<String>,
    pub utilisateur_id: Option<i32>,
    #[validate(length(
        min = 8,
        max = 30,
        message = "Password must be between 8 and 30 characters"
    ))]
    pub mot_de_passe: String,
    #[validate(must_match(
        other = "mot_de_passe",
        message = "Please ensure that your password is correctly entered in both fields"
    ))]
    pub confirmation: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct FosterUpdate {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your first name must usually be between 2 and 50 characters"
    ))]
    pub prenom: Option<Option<String>>,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your full last name should be between 2 and 50 characters"
    ))]
    pub nom: Option<String>,
    #[validate(custom(function = validate_phone))]
    pub telephone: Option<String>,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your address must usually be between 2 and 50 characters"
    ))]
    pub rue: Option<String>,
     #[validate(length(
        min = 2,
        max = 58,
        message = "Your city's name must be between 2 and 58 characters"
    ))]
    pub commune: Option<String>,
    #[validate(custom(function = validate_zipcode))]
    pub code_postal: Option<String>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe your home using between 3 and 50 characters"
    ))]
    pub hebergement: Option<String>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe your garden/yard using between 3 and 50 characters"
    ))]
    pub terrain: Option<Option<String>>,
    pub utilisateur_id: Option<i32>,
}
/* pub async fn get_fosters(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = FamilleRepository::new(db.get_ref());

    let fosters = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve fosters: {}", e)))?;

    Ok(HttpResponse::Ok().json(fosters))
}
*/
pub async fn get_foster(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, CustomError> {

    let foster_id = path.into_inner();
    let repo = FamilleRepository::new(db.get_ref());

    let foster = repo
        .find_by_id(foster_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    match foster {
        Some(foster) => Ok(HttpResponse::Ok().json(foster)),
        None => Err(CustomError::NotFound),
    }
}

pub async fn create_foster(
    db: web::Data<DbConn>,
    json_foster: web::Json<FosterCreate>,
) -> Result<HttpResponse, CustomError> {
    process_json_validation(&json_foster)?;

    info!(
        "Attempting to create Foster with name: {}",
        json_foster.nom
    );

    let user_repo = UtilisateurRepository::new(db.get_ref());

    if let Some(_) = user_repo
        .find_by_email(&json_foster.email)
        .await
        .map_err(|_e| CustomError::InternalError)?
    {
        return Err(CustomError::BadClientData);
    }

    let hashed_password = hash_password(&json_foster.mot_de_passe)?;

    let foster = json_foster.into_inner();

    let user_model = UtilisateurActiveModel {
        email: Set(foster.email),
        mot_de_passe: Set(hashed_password),
        ..Default::default()
    };

    let created_user = user_repo
        .create(user_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("User created with ID: {}", created_user.id);

    let repo = FamilleRepository::new(db.get_ref());

    let foster_model = FamilleActiveModel {
        prenom: Set(foster.prenom),
        nom: Set(foster.nom),
        telephone: Set(foster.telephone),
        rue: Set(foster.rue),
        commune: Set(foster.commune),
        code_postal: Set(foster.code_postal),
        pays: Set(foster.pays),
        hebergement: Set(foster.hebergement),
        terrain: Set(foster.terrain),
        utilisateur_id: Set(created_user.id),
        ..Default::default()
    };

    let created_foster = repo
        .create(foster_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Foster created with ID: {}", created_foster.id);
    Ok(HttpResponse::Created().json(created_foster))
}

pub async fn update_foster(
    db: web::Data<DbConn>,
    /* path: web::Path<i32>, */
    req: HttpRequest,
    json_foster: web::Json<FosterUpdate>,
) -> Result<HttpResponse, CustomError> {
    process_json_validation(&json_foster)?;

    let user_id = req.extensions_mut().get::<i32>().cloned().unwrap();
    
    let user_repo = UtilisateurRepository::new(db.get_ref());

    let user = user_repo
        .find_by_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if user.is_none() {
        return Err(CustomError::NotFound);
    }

    let repo = FamilleRepository::new(db.get_ref());

    let foster_data = repo
        .find_by_user_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;

    match foster_data {
        Some(foster_data) => {
            let mut foster_active_model: FamilleActiveModelEx = foster_data.into();

            let foster = json_foster.into_inner();

            if let Some(prenom) = foster.prenom {    
                foster_active_model.prenom = Set(prenom);
            }
            if let Some(nom) = foster.nom {
                foster_active_model.nom = Set(nom);
            }
            if let Some(telephone) = foster.telephone {
                foster_active_model.telephone = Set(telephone);
            }
            if let Some(rue) = foster.rue {
                foster_active_model.rue = Set(rue);
            }
            if let Some(commune) = foster.commune {
                foster_active_model.commune = Set(commune);
            }
            if let Some(code_postal) = foster.code_postal {
                foster_active_model.code_postal = Set(code_postal);
            }
            /* if let Some(pays) = foster.pays {
                foster_active_model.pays = Set(pays);
            } */
            if let Some(hebergement) = foster.hebergement {
                foster_active_model.hebergement = Set(hebergement);
            }
            if let Some(terrain) = foster.terrain {
                foster_active_model.terrain = Set(terrain);
            }

            let updated_foster = repo
                .update(foster_active_model)
                .await
                .map_err(|_e| CustomError::UpdateError)?;

            info!("Foster succesfully updated");
            Ok(HttpResponse::Ok().json(updated_foster))
        }
        None => Err(CustomError::NotFound),
    }
}

pub async fn delete_foster(
    db: web::Data<DbConn>,
    req: HttpRequest,
    /* path: web::Path<i32>, */
) -> Result<HttpResponse, CustomError> {

    let user_id = req.extensions_mut().get::<i32>().cloned().unwrap();
    
    let user_repo = UtilisateurRepository::new(db.get_ref());

    let user = user_repo
        .find_by_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if user.is_none() {
        return Err(CustomError::NotFound);
    }

    let repo = FamilleRepository::new(db.get_ref());

    let foster = repo
        .find_by_user_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if foster.is_none() {
        return Err(CustomError::NotFound);
    }
    if !foster.as_ref().unwrap().animals.is_empty() {
        return Err(CustomError::FosteredError);
    }

    let foster_id = foster.as_ref().unwrap().id;
    info!("Attempting to delete foster with ID: {}", foster_id);

    let delete_result = repo
        .delete(foster_id)
        .await
        .map_err(|_e| CustomError::DeletionError)?;

    let delete_user = user_repo
        .delete(user_id)
        .await
        .map_err(|_e| CustomError::DeletionError)?;

    if delete_result.rows_affected > 0 && delete_user.rows_affected > 0 {
        info!("Foster with ID {} successfully deleted", foster_id);
        info!("User with ID {} also successfully deleted", user_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Foster with ID {} was not deleted (0 rows affected)", foster_id);
        warn!("User with ID {} was not deleted (0 rows affected)", user_id);
        Err(CustomError::DeletionError)
    }
}