use actix_web::error::{ErrorInternalServerError, ErrorNotFound, /* ErrorUnprocessableEntity */};
use actix_web::{Error, HttpResponse, web};
use log::{info, /* warn */};
use sea_orm::DbConn;

use serde::{Deserialize, Serialize};

use crate::database::models::MediaActiveModel;
use crate::database::repositories::MediaRepository;

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_medias)
            .post(create_media)
        )
        /* .service(web::resource("/{id}")
            .get(get_media)
            .put(update_media)
            .delete(delete_media)
        ); */
        ;
}

#[derive(Deserialize, Serialize)]
pub struct MediaCreate {
    pub url: String,
    pub ordre: i32,
    pub animal_id: Option<i32>,
    pub association_id: Option<i32>,
}
/* #[derive(Deserialize, Serialize)]
pub struct MediaUpdate {
    pub url: Option<String>,
    pub ordre: Option<i32>,
    pub animal_id: Option<Option<i32>>,
    pub association_id: Option<Option<i32>>,
}
 */
pub async fn get_medias(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = MediaRepository::new(db.get_ref());

    let medias = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve medias: {}", e)))?;

    Ok(HttpResponse::Ok().json(medias))
}
/* pub async fn get_media(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let media_id = path.into_inner();
    let repo = MediaRepository::new(db.get_ref());

    let media = repo
        .find_by_id(media_id)
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve media: {}", e)))?;

    match media {
        Some(media) => Ok(HttpResponse::Ok().json(media)),
        None => Err(ErrorNotFound(format!("Media with ID {} not found", media_id))),
    }
}
 */
pub async fn create_media(
    db: web::Data<DbConn>,
    json_media: web::Json<MediaCreate>,
) -> Result<HttpResponse, Error> {

    info!(
        "Attempting to create media : {}",
        json_media.url
    );

    let repo = MediaRepository::new(db.get_ref());

    let media = json_media.into_inner();

    let media_model = MediaActiveModel {
        url: Set(media.url),
        ordre: Set(1),
        animal_id: Set(media.animal_id),
        association_id: Set(media.association_id),
        ..Default::default()
    };

    let created_media = repo
        .create(media_model)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to create media: {}", e)))?;

    info!("Media created with ID: {}", created_media.id);
    Ok(HttpResponse::Created().json(created_media))
}
/* pub async fn update_media(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_media: web::Json<MediaUpdate>,
) -> Result<HttpResponse, Error> {

    let media_id = path.into_inner();

    info!("Attempting to update media with ID: {}", media_id);

    let repo = MediaRepository::new(db.get_ref());

    let media_data = repo
        .find_by_id(media_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    match media_data {
        Some(media_data) => {
            let mut media_active_model: MediaActiveModel = media_data.into();

            let media = json_media.into_inner();

            if let Some(url) = media.url {
            media_active_model.url = Set(url);
            }
            if let Some(ordre) = media.ordre {
            media_active_model.ordre = Set(ordre);
            }
            if let Some(animal_id) = media.animal_id {
            media_active_model.animal_id = Set(animal_id);
            }
            if let Some(association_id) = media.association_id {
            media_active_model.association_id = Set(association_id);
            }

            let updated_media = repo
                .update(media_active_model)
                .await
                .map_err(|e| ErrorInternalServerError(format!("Failed to update media: {}", e)))?;

            info!("Media with ID {} updated", media_id);
            Ok(HttpResponse::Ok().json(updated_media))
        }
        None => Err(ErrorNotFound(format!("Media with ID {} not found", media_id))),
    }
}

pub async fn delete_media(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let media_id = path.into_inner();
    let repo = MediaRepository::new(db.get_ref());

    info!("Attempting to delete media with ID: {}", media_id);

    let media = repo
        .find_by_id(media_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?;
    if media.is_none() {
        return Err(ErrorNotFound(format!("Media with ID {} not found", media_id)));
    }

    let delete_result = repo
        .delete(media_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to delete media: {}", e)))?;

    if delete_result.rows_affected > 0 {
        info!("Media with ID {} successfully deleted", media_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Media with ID {} was not deleted (0 rows affected)", media_id);
        Err(ErrorInternalServerError(
            "Failed to delete meida (0 rows affected)",
        ))
    }
}
 */