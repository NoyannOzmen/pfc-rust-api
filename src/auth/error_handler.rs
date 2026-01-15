use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Serialize)]
struct FormattedErrorResponse {
    status_code: u16,
    error: String,
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display("Les champs suivants présentent des erreurs : {}", error_messages)]
    ValidationError { error_messages: String },
    #[display("Une erreur est survenue. Merci de réessayer ultérieurement.")]
    InternalError,
    #[display("Erreur lors de la création")]
    CreationError,
    #[display("Erreur lors de la mise à jour.")]
    UpdateError,
    #[display("Suppression impossible.")]
    DeletionError,
    #[display("Les informations saisies n'ont pas l'air correctes. Merci de réessayer.")]
    BadClientData,
    #[display("Vous avez déjà effectué une demande pour cet animal !")]
    AlreadyRequested,
    #[display("Vous accueillez actuellement un ou plusieurs animaux enregistrés sur notre site. Merci de contacter un administrateur afin de supprimer votre compte !")]
    ShelteredError,
    #[display("Vous accueillez actuellement un animal. Merci de contacter le refuge concerné avant de supprimer votre compte !")]
    FosteredError,
    #[display("Ce que vous recherchez n'a pas l'air d'exister.")]
    NotFound,
    #[display("Les informations saisies n'ont pas l'air correctes. Merci de réessayer.")]
    WrongLogin,
}

impl CustomError {
    fn name(&self) -> String {
        match self {
            CustomError::ValidationError { .. } => "Validation Error".to_string(),
            CustomError::InternalError => "Internal Server Error".to_string(),
            CustomError::CreationError => "Creation Failed".to_string(),
            CustomError::UpdateError => "Update Failed".to_string(),
            CustomError::DeletionError => "Could not delete".to_string(),
            CustomError::BadClientData => "Bad Request".to_string(),
            CustomError::AlreadyRequested => "Already requested".to_string(),
            CustomError::FosteredError => "Still fostering".to_string(),
            CustomError::ShelteredError => "Still sheltering".to_string(),
            CustomError::NotFound => "Not Found".to_string(),
            CustomError::WrongLogin => "Invalid Credentials".to_string(),
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let error_response = FormattedErrorResponse {
            status_code: self.status_code().as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::CreationError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::UpdateError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::DeletionError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomError::BadClientData => StatusCode::BAD_REQUEST,
            CustomError::AlreadyRequested => StatusCode::BAD_REQUEST,
            CustomError::FosteredError => StatusCode::BAD_REQUEST,
            CustomError::ShelteredError => StatusCode::BAD_REQUEST,
            CustomError::NotFound => StatusCode::NOT_FOUND,
            CustomError::WrongLogin => StatusCode::UNAUTHORIZED,
        }
    }
}