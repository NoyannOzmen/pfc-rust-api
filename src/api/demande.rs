use actix_web::error::{ErrorInternalServerError, ErrorNotFound, /* ErrorUnprocessableEntity */};
use actix_web::{Error, HttpResponse, web};
use log::{info, /* warn */};
use sea_orm::DbConn;

use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};

use crate::database::models::DemandeActiveModel;
use crate::database::models::sea_orm_active_enums::StatutDemande;
use crate::database::repositories::DemandeRepository;

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_current_requests)
            .post(create_request)
        )
        .service(web::resource("/{id}")
            .get(get_request)
            /* .put(update_request)
            .delete(delete_request) */
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


/* #[derive(Deserialize, Serialize)]
    pub struct DemandeUpdate {
    pub famille_id: Option<i32>,
    pub animal_id: Option<i32>,
    pub statut_demande: Option<StatutDemande>,
    pub date_debut: Option<Date>,
    pub date_fin: Option<Date>,
} */


/* pub async fn get_requests(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = DemandeRepository::new(db.get_ref());

    let requests = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve requests: {}", e)))?;

    Ok(HttpResponse::Ok().json(requests))
}
 */
pub async fn get_current_requests(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = DemandeRepository::new(db.get_ref());

    let requests = repo
        .find_current_requests()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve requests: {}", e)))?;

    Ok(HttpResponse::Ok().json(requests))
}

pub async fn get_request(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let request_id = path.into_inner();
    let repo = DemandeRepository::new(db.get_ref());

    let request = repo
        .find_by_id(request_id)
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve request: {}", e)))?;

    match request {
        Some(request) => Ok(HttpResponse::Ok().json(request)),
        None => Err(ErrorNotFound(format!("Request with ID {} not found", request_id))),
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

    //TODO ADD BREAK if already made a request */

    let created_request = repo
        .create(request_model)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to create request: {}", e)))?;

    info!("Request created with ID: {}", created_request.id);
    Ok(HttpResponse::Created().json(created_request))
}

/* pub async fn update_request(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_request: web::Json<DemandeUpdate>,
) -> Result<HttpResponse, Error> {

    let request_id = path.into_inner();

    info!("Attempting to update request with ID: {}", request_id);

    let repo = DemandeRepository::new(db.get_ref());

    let request_data = repo
        .find_by_id(request_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    match request_data {
        Some(request_data) => {
            let mut request_active_model: DemandeActiveModel = request_data.into();

            let request = json_request.into_inner();

            if let Some(famille_id) = request.famille_id {
              request_active_model.famille_id = Set(famille_id);
            }
            if let Some(animal_id) = request.animal_id {
              request_active_model.animal_id = Set(animal_id);
            }
            if let Some(statut_demande) = request.statut_demande {
              request_active_model.statut_demande = Set(statut_demande);
            }
            if let Some(date_debut) = request.date_debut {
              request_active_model.date_debut = Set(date_debut);
            }
            if let Some(date_fin) = request.date_fin {
              request_active_model.date_fin = Set(date_fin);
            }

            let updated_request = repo
                .update(request_active_model)
                .await
                .map_err(|e| ErrorInternalServerError(format!("Failed to update request: {}", e)))?;

            info!("Request with ID {} updated", request_id);
            Ok(HttpResponse::Ok().json(updated_request))
        }
        None => Err(ErrorNotFound(format!("Request with ID {} not found", request_id))),
    }
}

pub async fn delete_request(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let request_id = path.into_inner();
    let repo = DemandeRepository::new(db.get_ref());

    info!("Attempting to delete request with ID: {}", request_id);

    let request = repo
        .find_by_id(request_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?;
    if request.is_none() {
        return Err(ErrorNotFound(format!("Request with ID {} not found", request_id)));
    }

    let delete_result = repo
        .delete(request_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to delete request: {}", e)))?;

    if delete_result.rows_affected > 0 {
        info!("Request with ID {} successfully deleted", request_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Request with ID {} was not deleted (0 rows affected)", request_id);
        Err(ErrorInternalServerError(
            "Failed to delete request (0 rows affected)",
        ))
    }
} */
