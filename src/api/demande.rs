use actix_web::{Error, HttpMessage as _, HttpRequest, HttpResponse, web};
use log::info;
use sea_orm::DbConn;

use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};

use crate::auth::CustomError;
use crate::database::models::DemandeActiveModel;
use crate::database::models::sea_orm_active_enums::StatutDemande;
use crate::database::repositories::{DemandeRepository, FamilleRepository};

use sea_orm::ActiveValue::Set;

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_current_requests)
            .post(create_request)
        )
        .service(web::resource("/{id}")
            .get(get_request)
        );
}

#[derive(Deserialize, Serialize)]
pub struct DemandeCreate {
    pub famille_id: i32,
    pub animal_id: i32,
    pub statut_demande: StatutDemande,
    pub date_debut: Date,
    pub date_fin: Date,
}

pub async fn get_current_requests(db: web::Data<DbConn>, req: HttpRequest) -> Result<HttpResponse, CustomError> {

    let user_id = req.extensions_mut().get::<i32>().cloned().unwrap();

    let repo = FamilleRepository::new(db.get_ref());

    let foster = repo
        .find_by_user_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if foster.is_none() {
        return Err(CustomError::NotFound);
    }

    let foster_id = foster.as_ref().unwrap().id;

    let repo = DemandeRepository::new(db.get_ref());

    let requests = repo
        .find_current_requests(foster_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(requests))
}

pub async fn get_request(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, CustomError> {
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

pub async fn create_request(
    db: web::Data<DbConn>,
    json_request: web::Json<DemandeCreate>,
) -> Result<HttpResponse, Error> {

    info!(
        "Attempting to create request : {}",
        json_request.date_debut
    );

    let repo = DemandeRepository::new(db.get_ref());

    let request = json_request.into_inner();

    let request_model = DemandeActiveModel {
        famille_id: Set(request.famille_id),
        animal_id: Set(request.animal_id),
        statut_demande: Set(request.statut_demande),
        date_debut: Set(request.date_debut),
        date_fin: Set(request.date_fin),
        ..Default::default()
    };

    let created_request = repo
        .create(request_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Request created with ID: {}", created_request.id);
    Ok(HttpResponse::Created().json(created_request))
}