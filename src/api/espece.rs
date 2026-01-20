use actix_web::{Error, HttpResponse, web};
use sea_orm::DbConn;
use crate::auth::CustomError;

use crate::database::repositories::EspeceRepository;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_all_species)
        );
}

pub async fn get_all_species(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = EspeceRepository::new(db.get_ref());

    let species = repo
        .find_all()
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(species))
}