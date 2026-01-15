/* use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnprocessableEntity}; */
use actix_web::{Error, HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;
use validator::Validate;

use serde::{Deserialize, Serialize};

use crate::auth::{CustomError, hash_password};
use crate::database::models::{AssociationActiveModel, AssociationActiveModelEx, DemandeActiveModelEx, UtilisateurActiveModel};
use crate::database::repositories::{AnimalRepository, AssociationRepository, DemandeRepository, UtilisateurRepository};
use crate::database::models::sea_orm_active_enums::StatutDemande::*;
use crate::validators::common_validators::{process_json_validation, validate_phone, validate_siret, validate_zipcode};

use sea_orm::ActiveValue::Set;

pub fn configure_register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .post(create_shelter)
        );
}

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_shelters)
        )
        .service(web::resource("/{id}")
            .get(get_shelter)
        )
        .service(web::resource("/{id}/fostered")
            .get(get_fostered)
        )
        .service(web::resource("/{id}/requested")
            .get(get_requested)
        );
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .post(update_shelter)
        )
        .service(web::resource("/{id}")
            .get(get_shelter)
        )
        .service(web::resource("/delete")
            .delete(delete_shelter)
        )
        .service(web::resource("/animaux/{id}")
            .get(get_resident_details)
        )
        .service(web::resource("/demandes/{id}")
            .get(get_request_details)
        )
        .service(web::resource("/demandes/{id}/accept")
            .post(accept_request)
        )
        .service(web::resource("/demandes/{id}/deny")
            .post(deny_request)
        )
       ;
}

#[derive(Deserialize, Serialize, Validate)]
pub struct AssociationCreate {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your shelter's name must usually be between 2 and 50 characters"
    ))]
    pub nom: String,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Their full name must usually be between 2 and 50 characters"
    ))]
    pub responsable: String,
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
    #[validate(custom(function = validate_siret))]
    pub siret: String,
    #[validate(custom(function = validate_phone))]
    pub telephone: String,
    #[validate(url)]
    pub site: Option<String>,
    #[validate(length(
        min = 3,
        max = 200,
        message = "Please describe your shelter using between 3 and 200 characters"
    ))]
    pub description: Option<String>,
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
pub struct AssociationUpdate {
    #[validate(length(
        min = 2,
        max = 50,
        message = "Your shelter's name must usually be between 2 and 50 characters"
    ))]
    pub nom: Option<String>,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Their full name must usually be between 2 and 50 characters"
    ))]
    pub responsable: Option<String>,
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
        min = 4,
        max = 42,
        message = "Country name must be between 4 and 42 characters"
    ))]
    pub pays: Option<String>,
    #[validate(custom(function = validate_siret))]
    pub siret: Option<String>,
    #[validate(custom(function = validate_phone))]
    pub telephone: Option<String>,
    #[validate(url)]
    pub site: Option<Option<String>>,
    #[validate(length(
        min = 3,
        max = 200,
        message = "Please describe your shelter using between 3 and 200 characters"
    ))]
    pub description: Option<Option<String>>,
    pub utilisateur_id: Option<i32>,
}

pub async fn get_shelters(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = AssociationRepository::new(db.get_ref());

    let shelters = repo
        .find_all()
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(shelters))
}

pub async fn get_shelter(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let shelter_id = path.into_inner();
    let repo = AssociationRepository::new(db.get_ref());

    let shelter = repo
        .find_by_id(shelter_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;
    
    match shelter {
        Some(shelter) => Ok(HttpResponse::Ok().json(shelter)),
        None => Err(CustomError::NotFound)
    }
}

pub async fn create_shelter(
    db: web::Data<DbConn>,
    json_shelter: web::Json<AssociationCreate>,
) -> Result<HttpResponse, CustomError> {
    process_json_validation(&json_shelter)?;

    info!(
        "Attempting to create shelter with name: {}",
        json_shelter.nom
    );

    let user_repo = UtilisateurRepository::new(db.get_ref());

    if let Some(_) = user_repo
        .find_by_email(&json_shelter.email)
        .await
        .map_err(|_e| CustomError::InternalError)?
    {
        return Err(CustomError::BadClientData);
    }

    let hashed_password = hash_password(&json_shelter.mot_de_passe)?;

    let shelter = json_shelter.into_inner();

    let user_model = UtilisateurActiveModel {
        email: Set(shelter.email),
        mot_de_passe: Set(hashed_password),
        ..Default::default()
    };

    let created_user = user_repo
        .create(user_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("User created with ID: {}", created_user.id);

    let repo = AssociationRepository::new(db.get_ref());

    let shelter_model = AssociationActiveModel {
        nom: Set(shelter.nom),
        responsable: Set(shelter.responsable),
        rue: Set(shelter.rue),
        commune: Set(shelter.commune),
        code_postal: Set(shelter.code_postal),
        pays: Set(shelter.pays),
        siret: Set(shelter.siret),
        telephone: Set(shelter.telephone),
        site: Set(shelter.site),
        description: Set(shelter.description),
        utilisateur_id: Set(created_user.id),
        ..Default::default()
    };

    let created_shelter = repo
        .create(shelter_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Shelter created with ID: {}", created_shelter.id);
    Ok(HttpResponse::Created().json(created_shelter))
}

pub async fn update_shelter(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_shelter: web::Json<AssociationUpdate>,
) -> Result<HttpResponse, CustomError> {
    process_json_validation(&json_shelter)?;

    let shelter_id = path.into_inner();

    info!("Attempting to update shelter with ID: {}", shelter_id);

    let repo = AssociationRepository::new(db.get_ref());

    let shelter_data = repo
        .find_by_id(shelter_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;

    match shelter_data {
        Some(shelter_data) => {
            let mut shelter_active_model: AssociationActiveModelEx = shelter_data.into();

            let shelter = json_shelter.into_inner();

            if let Some(nom) = shelter.nom {
                shelter_active_model.nom = Set(nom);
            }
            if let Some(responsable) = shelter.responsable {
                shelter_active_model.responsable = Set(responsable);
            }
            if let Some(rue) = shelter.rue {
                shelter_active_model.rue= Set(rue);
            }
            if let Some(commune) = shelter.commune {
                shelter_active_model.commune = Set(commune);
            }
            if let Some(code_postal) = shelter.code_postal {
                shelter_active_model.code_postal = Set(code_postal);
            }
            if let Some(pays) = shelter.pays {
                shelter_active_model.pays = Set(pays);
            }
            if let Some(siret) = shelter.siret {
                shelter_active_model.siret = Set(siret);
            }
            if let Some(telephone) = shelter.telephone {
                shelter_active_model.telephone = Set(telephone);
            }
            if let Some(site) = shelter.site {
                shelter_active_model.site = Set(site);
            }
            if let Some(description) = shelter.description {
                shelter_active_model.description = Set(description);
            }

            let updated_shelter = repo
                .update(shelter_active_model)
                .await
                .map_err(|_e| CustomError::UpdateError)?;

            info!("Shelter with ID {} updated", shelter_id);
            Ok(HttpResponse::Ok().json(updated_shelter))
        }
        None => Err(CustomError::NotFound),
    }
}

pub async fn delete_shelter(
    db: web::Data<DbConn>,
    /* path: web::Path<i32>, */
) -> Result<HttpResponse, CustomError> {

    //TODO REMOVE HARDCODED */
    /* let shelter_id = path.into_inner(); */
    let shelter_id = 5;
    let repo = AssociationRepository::new(db.get_ref());

    info!("Attempting to delete shelter with ID: {}", shelter_id);

    let shelter = repo
        .find_by_id(shelter_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if shelter.is_none() {
        return Err(CustomError::NotFound);
    }
    if !shelter.unwrap().pensionnaires.is_empty() {
        return Err(CustomError::ShelteredError);
    }

    //TODO REMOVE HARDCODED */
    let user_id = 10;

    let user_repo = UtilisateurRepository::new(db.get_ref());

    let user = user_repo
        .find_by_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if user.is_none() {
        return Err(CustomError::NotFound);
    }

    let delete_result = repo
        .delete(shelter_id)
        .await
        .map_err(|_e| CustomError::DeletionError)?;

    let delete_user = user_repo
        .delete(user_id)
        .await
        .map_err(|_e| CustomError::DeletionError)?;


    if delete_result.rows_affected > 0  && delete_user.rows_affected > 0 {
        info!("Shelter with ID {} successfully deleted", shelter_id);
        info!("User with ID {} also successfully deleted", user_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Shelter with ID {} was not deleted (0 rows affected)", shelter_id);
        warn!("User with ID {} was not deleted (0 rows affected)", user_id);
        Err(CustomError::InternalError)
    }
}

pub async fn get_resident_details(
    db: web::Data<DbConn>,
    path: web::Path<i32>
) -> Result<HttpResponse, CustomError> {
    let animal_id = path.into_inner();
    let repo = AnimalRepository::new(db.get_ref());

    let animal = repo
        .find_by_id(animal_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    match animal {
        Some(animal) => Ok(HttpResponse::Ok().json(animal)),
        None => Err(CustomError::NotFound),
    }
}

pub async fn get_request_details(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let request_id = path.into_inner();
    let repo = DemandeRepository::new(db.get_ref());

    let request = repo
        .find_by_id(request_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    match request {
        Some(request) => Ok(HttpResponse::Ok().json(request)),
        None => Err(CustomError::NotFound),
    }
}

pub async fn accept_request(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {

    let request_id = path.into_inner();

    info!("Attempting to accept request with ID: {}", request_id);

    let repo = DemandeRepository::new(db.get_ref());

    let request_data = repo
        .find_by_id(request_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;

    match request_data {
        Some(request_data) => {
            let mut request_active_model: DemandeActiveModelEx = request_data.into();

            request_active_model.statut_demande = Set(Validée);

            let updated_request = repo
                .update(request_active_model)
                .await
                .map_err(|_e| CustomError::UpdateError)?;

            info!("Accepted request with ID {}", request_id);
            Ok(HttpResponse::Ok().json(updated_request))
        }
        None => Err(CustomError::NotFound),
    }
}

pub async fn deny_request(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {

    let request_id = path.into_inner();

    info!("Attempting to deny request with ID: {}", request_id);

    let repo = DemandeRepository::new(db.get_ref());

    let request_data = repo
        .find_by_id(request_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;

    match request_data {
        Some(request_data) => {
            let mut request_active_model: DemandeActiveModelEx = request_data.into();

            request_active_model.statut_demande = Set(Refusée);

            let updated_request = repo
                .update(request_active_model)
                .await
                .map_err(|_e| CustomError::UpdateError)?;

            info!("Denied request with ID {}", request_id);
            Ok(HttpResponse::Ok().json(updated_request))
        }
        None => Err(CustomError::NotFound),
    }
}

pub async fn get_fostered(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let shelter_id = path.into_inner();

    let repo = AnimalRepository::new(db.get_ref());
    info!("Attempting to find currently fostered animals");

    let fostered = repo
        .find_fostered(shelter_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(fostered))
}

pub async fn get_requested(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let shelter_id = path.into_inner();

    let repo = AnimalRepository::new(db.get_ref());
    info!("Attempting to find currently requested animals");

    let requested = repo
        .find_requested(shelter_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(requested))
}
