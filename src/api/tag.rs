/* use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnprocessableEntity }; */
use actix_web::{Error, HttpResponse, web};
use log::{info, /* warn */};
use sea_orm::DbConn;
use validator::Validate;

use serde::{Deserialize, Serialize};

use crate::auth::CustomError;
use crate::database::models::TagActiveModel;
use crate::database::repositories::TagRepository;
use crate::validators::common_validators::{process_json_validation};

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_tags)
        )
        /* .service(web::resource("/create")
            .post(create_tag)
        ) */
        .service(web::resource("/{id}")
            .get(get_tag)
            /* .put(update_tag)
            .delete(delete_tag) */
        );
}

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .post(create_tag)
        );
}

#[derive(Deserialize, Serialize, Validate)]
pub struct TagCreate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe this tag using between 3 and 50 characters"
    ))]
    pub description: String,
}

/* #[derive(Deserialize, Serialize, Validate)]
pub struct TagUpdate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom: Option<String>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe this tag using between 3 and 50 characters"
    ))]
    pub description: Option<String>,
} */

pub async fn get_tags(db: web::Data<DbConn>) -> Result<HttpResponse, CustomError> {
    let repo = TagRepository::new(db.get_ref());

    let tags = repo
        .find_all()
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(tags))
}

pub async fn get_tag(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let tag_id = path.into_inner();
    let repo = TagRepository::new(db.get_ref());

    let tag = repo
        .find_by_id(tag_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    match tag {
        Some(tag) => Ok(HttpResponse::Ok().json(tag)),
        None => Err(CustomError::NotFound),
    }
}

pub async fn create_tag(
    db: web::Data<DbConn>,
    json_tag: web::Json<TagCreate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_tag)?;

    info!(
        "Attempting to create tag with name: {}",
        json_tag.nom
    );

    let repo = TagRepository::new(db.get_ref());

    let tag = json_tag.into_inner();

    let tag_model = TagActiveModel {
        nom: Set(tag.nom),
        description: Set(tag.description),
        ..Default::default()
    };

    let created_tag = repo
        .create(tag_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Tag created with ID: {}", created_tag.id);
    Ok(HttpResponse::Created().json(created_tag))
}
/* 
pub async fn update_tag(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_tag: web::Json<TagUpdate>,
) -> Result<HttpResponse, CustomError> {

    let tag_id = path.into_inner();

    info!("Attempting to update Tag with ID: {}", tag_id);

    let repo = TagRepository::new(db.get_ref());

    let tag_data = repo
        .find_by_id(tag_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;

    match tag_data {
        Some(tag_data) => {
            let mut tag_active_model: TagActiveModel = tag_data.into();

            let tag = json_tag.into_inner();

            if let Some(nom) = tag.nom {
                tag_active_model.nom = Set(nom);
            }

            if let Some(description) = tag.description {
                tag_active_model.description = Set(description);
            }

            let updated_tag = repo
                .update(tag_active_model)
                .await
                .map_err(|_e| CustomError::UpdateError)?;

            info!("Tag with ID {} updated", tag_id);
            Ok(HttpResponse::Ok().json(updated_tag))
        }
        None => Err(CustomError::NotFound),
    }
}

pub async fn delete_tag(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let tag_id = path.into_inner();
    let repo = TagRepository::new(db.get_ref());

    info!("Attempting to delete tag with ID: {}", tag_id);

    let tag = repo
        .find_by_id(tag_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if tag.is_none() {
        return Err(CustomError::NotFound);
    }

    let delete_result = repo
        .delete(tag_id)
        .await
        .map_err(|_e| CustomError::DeletionError)?;

    if delete_result.rows_affected > 0 {
        info!("Tag with ID {} successfully deleted", tag_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Tag with ID {} was not deleted (0 rows affected)", tag_id);
        Err(CustomError::DeletionError)
    }
} */