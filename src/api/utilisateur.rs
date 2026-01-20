use actix_web::{HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;

use crate::auth::CustomError;
use crate::database::repositories::UtilisateurRepository;


pub fn configure_protected(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{id}")
            .delete(delete_user)
        );
}

pub async fn delete_user(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let user_id = path.into_inner();
    let repo = UtilisateurRepository::new(db.get_ref());

    info!("Attempting to delete user with ID: {}", user_id);

    let user = repo
        .find_by_id(user_id)
        .await
        .map_err(|_e| CustomError::InternalError)?;
    if user.is_none() {
        return Err(CustomError::NotFound);
    }

    let delete_result = repo
        .delete(user_id)
        .await
        .map_err(|_e| CustomError::DeletionError)?;

    if delete_result.rows_affected > 0 {
        info!("User with ID {} successfully deleted", user_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("User with ID {} was not deleted (0 rows affected)", user_id);
        Err(CustomError::DeletionError)
    }
}