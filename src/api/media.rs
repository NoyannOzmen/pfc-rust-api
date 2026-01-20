use actix_multipart::form::text::Text;
use actix_web::{Error, HttpResponse, web};
use actix_web::{Responder};
use actix_multipart::{
    form::{
        MultipartForm,
        tempfile::{TempFile},
    },
};
use log::{info, warn};
use sea_orm::DbConn;

use crate::auth::CustomError;
use crate::database::models::MediaActiveModel;
use crate::database::repositories::{MediaRepository};

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_medias)
        );
}

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/logo")
            .post(upload_logo)
        )
        .service(web::resource("/photo")
            .post(upload_photo)
        );
}

pub async fn get_medias(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = MediaRepository::new(db.get_ref());

    let medias = repo
        .find_all()
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(medias))
}

#[derive(Debug, MultipartForm)]
pub struct LogoUploadForm {
    #[multipart(limit = "5MB")]
    file: TempFile,
    asso_id: Option<Text<i32>>
}

pub async fn upload_logo(
    db: web::Data<DbConn>,
    MultipartForm(form): MultipartForm<LogoUploadForm>,
) -> Result<HttpResponse, CustomError> {

    let file_path = format!("/images/animaux/{}", form.file.file_name.unwrap());
    warn!("Saving picture to ./static{}", file_path);
    form.file.file.persist(format!("./static{}", file_path)).unwrap();
    warn!("Logo uploaded successfully");
    
    let repo = MediaRepository::new(db.get_ref());

    let shelter_id  = form.asso_id.unwrap().to_string();
    let id = shelter_id.parse::<i32>().unwrap();

    let media_model = MediaActiveModel {
        url: Set(file_path),
        ordre: Set(1),
        association_id: Set(Some(id)),
        ..Default::default()
    };

    let created_media = repo
        .create(media_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Logo uploaded with ID: {}", created_media.id);
    Ok(HttpResponse::Created().json(created_media))
}

#[derive(Debug, MultipartForm)]
pub struct PhotoUploadForm {
    #[multipart(limit = "5MB")]
    file: TempFile,
    animal_id: Option<Text<i32>>
}

async fn upload_photo(
    db: web::Data<DbConn>,
    MultipartForm(form): MultipartForm<PhotoUploadForm>,
) -> Result<impl Responder, Error> {

    let file_path = format!("/images/animaux/{}", form.file.file_name.unwrap());
    warn!("Saving picture to ./static{}", file_path);
    form.file.file.persist(format!("./static{}", file_path)).unwrap();
    warn!("Logo uploaded successfully");

    let repo = MediaRepository::new(db.get_ref());

    let animal_id  = form.animal_id.unwrap().to_string();
    let id = animal_id.parse::<i32>().unwrap();

    let media_model = MediaActiveModel {
        url: Set(file_path),
        ordre: Set(1),
        animal_id: Set(Some(id)),
        ..Default::default()
    };

    let created_media = repo
        .create(media_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Logo uploaded with ID: {}", created_media.id);
    Ok(HttpResponse::Created().json(created_media))
}