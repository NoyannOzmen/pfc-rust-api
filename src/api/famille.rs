use actix_web::error::{ErrorInternalServerError, ErrorNotFound, /* ErrorUnprocessableEntity */};
use actix_web::{Error, HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;

use serde::{Deserialize, Serialize};

use crate::database::models::FamilleActiveModel;
use crate::database::repositories::FamilleRepository;

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_fosters)
            .post(create_foster)
        )
        .service(web::resource("/{id}")
            .get(get_foster)
            .put(update_foster)
            .delete(delete_foster)
        );
}

#[derive(Deserialize, Serialize)]
pub struct FosterCreate {
    pub prenom: Option<String>,
    pub nom: String,
    pub telephone: String,
    pub rue: String,
    pub commune: String,
    pub code_postal: String,
    pub pays: String,
    pub hebergement: String,
    pub terrain: Option<String>,
    pub utilisateur_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct FosterUpdate {
    pub prenom: Option<Option<String>>,
    pub nom: Option<String>,
    pub telephone: Option<String>,
    pub rue: Option<String>,
    pub commune: Option<String>,
    pub code_postal: Option<String>,
    pub pays: Option<String>,
    pub hebergement: Option<String>,
    pub terrain: Option<Option<String>>,
    pub utilisateur_id: Option<i32>,
}

pub async fn get_fosters(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = FamilleRepository::new(db.get_ref());

    let fosters = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve fosters: {}", e)))?;

    Ok(HttpResponse::Ok().json(fosters))
}

pub async fn get_foster(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let foster_id = path.into_inner();
    let repo = FamilleRepository::new(db.get_ref());

    let foster = repo
        .find_by_id(foster_id)
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve foster: {}", e)))?;

    match foster {
        Some(foster) => Ok(HttpResponse::Ok().json(foster)),
        None => Err(ErrorNotFound(format!("Foster with ID {} not found", foster_id))),
    }
}

pub async fn create_foster(
    db: web::Data<DbConn>,
    json_foster: web::Json<FosterCreate>,
) -> Result<HttpResponse, Error> {

    info!(
        "Attempting to create Foster with name: {}",
        json_foster.nom
    );

    let repo = FamilleRepository::new(db.get_ref());

    let foster = json_foster.into_inner();

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
        utilisateur_id: Set(foster.utilisateur_id),
        ..Default::default()
    };

    let created_foster = repo
        .create(foster_model)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to create foster: {}", e)))?;

    info!("Foster created with ID: {}", created_foster.id);
    Ok(HttpResponse::Created().json(created_foster))
}

pub async fn update_foster(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_foster: web::Json<FosterUpdate>,
) -> Result<HttpResponse, Error> {

    let foster_id = path.into_inner();

    info!("Attempting to update Foster with ID: {}", foster_id);

    let repo = FamilleRepository::new(db.get_ref());


    let foster_data = repo
        .find_by_id(foster_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    match foster_data {
        Some(foster_data) => {
            let mut foster_active_model: FamilleActiveModel = foster_data.into();

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
            if let Some(pays) = foster.pays {
                foster_active_model.pays = Set(pays);
            }
            if let Some(hebergement) = foster.hebergement {
                foster_active_model.hebergement = Set(hebergement);
            }
            if let Some(terrain) = foster.terrain {
                foster_active_model.terrain = Set(terrain);
            }

            let updated_foster = repo
                .update(foster_active_model)
                .await
                .map_err(|e| ErrorInternalServerError(format!("Failed to update foster: {}", e)))?;

            info!("Foster with ID {} updated", foster_id);
            Ok(HttpResponse::Ok().json(updated_foster))
        }
        None => Err(ErrorNotFound(format!("Foster with ID {} not found", foster_id))),
    }
}

pub async fn delete_foster(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let foster_id = path.into_inner();
    let repo = FamilleRepository::new(db.get_ref());

    info!("Attempting to delete foster with ID: {}", foster_id);

    let foster = repo
        .find_by_id(foster_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?;
    if foster.is_none() {
        return Err(ErrorNotFound(format!("Foster with ID {} not found", foster_id)));
    }

    let delete_result = repo
        .delete(foster_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to delete foster: {}", e)))?;

    if delete_result.rows_affected > 0 {
        info!("Foster with ID {} successfully deleted", foster_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Foster with ID {} was not deleted (0 rows affected)", foster_id);
        Err(ErrorInternalServerError(
            "Failed to delete foster (0 rows affected)",
        ))
    }
}