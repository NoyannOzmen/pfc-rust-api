use actix_web::error::{ErrorInternalServerError, ErrorNotFound, /* ErrorUnprocessableEntity */};
use actix_web::{Error, HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;
use validator::Validate;

use serde::{Deserialize, Serialize};

use crate::database::models::EspeceActiveModel;
use crate::database::repositories::EspeceRepository;
use crate::validators::common_validators::{process_json_validation};

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_all_species)
            .post(create_species)
        )
        .service(web::resource("/{id}")
            .get(get_species)
            .put(update_species)
            .delete(delete_species)
        );
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SpeciesCreate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SpeciesUpdate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom: Option<String>,
}

pub async fn get_all_species(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = EspeceRepository::new(db.get_ref());

    let species = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve species: {}", e)))?;

    Ok(HttpResponse::Ok().json(species))
}

pub async fn get_species(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let species_id = path.into_inner();
    let repo = EspeceRepository::new(db.get_ref());

    let species = repo
        .find_by_id(species_id)
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve species: {}", e)))?;

    match species {
        Some(species) => Ok(HttpResponse::Ok().json(species)),
        None => Err(ErrorNotFound(format!("Species with ID {} not found", species_id))),
    }
}

pub async fn create_species(
    db: web::Data<DbConn>,
    json_species: web::Json<SpeciesCreate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_species)?;

    info!(
        "Attempting to create species with name: {}",
        json_species.nom
    );

    let repo = EspeceRepository::new(db.get_ref());

    let species = json_species.into_inner();

    let species_model = EspeceActiveModel {
        nom: Set(species.nom),
        ..Default::default()
    };

    let created_species = repo
        .create(species_model)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to create species: {}", e)))?;

    info!("Species created with ID: {}", created_species.id);
    Ok(HttpResponse::Created().json(created_species))
}

pub async fn update_species(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_species: web::Json<SpeciesUpdate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_species)?;

    let species_id = path.into_inner();

    info!("Attempting to update species with ID: {}", species_id);

    let repo = EspeceRepository::new(db.get_ref());

    let species_data = repo
        .find_by_id(species_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    match species_data {
        Some(species_data) => {
            let mut species_active_model: EspeceActiveModel = species_data.into();

            let species = json_species.into_inner();

            if let Some(nom) = species.nom {
                species_active_model.nom = Set(nom);
            }

            let updated_species = repo
                .update(species_active_model)
                .await
                .map_err(|e| ErrorInternalServerError(format!("Failed to update species: {}", e)))?;

            info!("Species with ID {} updated", species_id);
            Ok(HttpResponse::Ok().json(updated_species))
        }
        None => Err(ErrorNotFound(format!("Species with ID {} not found", species_id))),
    }
}

pub async fn delete_species(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let species_id = path.into_inner();
    let repo = EspeceRepository::new(db.get_ref());

    info!("Attempting to delete species with ID: {}", species_id);

    let species = repo
        .find_by_id(species_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?;
    if species.is_none() {
        return Err(ErrorNotFound(format!("Species with ID {} not found", species_id)));
    }

    let delete_result = repo
        .delete(species_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to delete species: {}", e)))?;

    if delete_result.rows_affected > 0 {
        info!("Species with ID {} successfully deleted", species_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Species with ID {} was not deleted (0 rows affected)", species_id);
        Err(ErrorInternalServerError(
            "Failed to delete species (0 rows affected)",
        ))
    }
}