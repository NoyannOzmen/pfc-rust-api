/* use actix_web::error::{ErrorInternalServerError, ErrorNotFound,  ErrorUnprocessableEntity }; */
use actix_web::{Error, HttpResponse, web};
use chrono::{Duration, Local};
use log::{info, /* warn */};
use sea_orm::{DbConn};
use validator::Validate;

use serde::{Deserialize, Serialize};

use crate::auth::CustomError;
use crate::database::models::{AnimalActiveModel, AnimalTagActiveModel, DemandeActiveModel};
use crate::database::models::sea_orm_active_enums::{Sexe, Statut, StatutDemande};
use crate::database::repositories::{AnimalRepository, AnimalTagRepository, DemandeRepository};
use crate::validators::common_validators::{process_json_validation};

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_animals)
            /* .post(create_animal) */
        )
        .service(web::resource("/nouveau-profil")
            .post(create_animal)
        )
        .service(web::resource("/{id}")
            .get(get_animal)
            /* .put(update_animal)
            .delete(delete_animal) */
        )
        .service(web::resource("/{id}/requests")
            .get(get_requests)
        )
        .service(web::resource("/{id}/faire-une-demande")
            .post(request_animal)
        );
}

#[derive(Deserialize, Serialize, Validate)]
pub struct AnimalCreate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom_animal: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "This must be between 3 and 50 characters"
    ))]
    pub race_animal: Option<String>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Colour name must be between 3 and 50 characters"
    ))]
    pub couleur_animal: String,
    #[validate(range(
        min = 1,
        max = 100,
        message = "Age must be realistic"
    ))]
    pub age_animal: i32,
    pub sexe_animal : Sexe,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe this animal using between 3 and 50 characters"
    ))]
    pub description_animal: String,
    /* pub statut: Statut, */
    pub association_id: i32,
    /* pub famille_id: Option<i32>, */
    pub espece_animal: i32,
    pub tags: Vec<i32>,
}
/* 
#[derive(Deserialize, Serialize, Validate)]
pub struct AnimalUpdate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom: Option<String>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "This must be between 3 and 50 characters"
    ))]
    pub race: Option<Option<String>>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Colour name must be between 3 and 50 characters"
    ))]
    pub couleur: Option<String>,
    #[validate(range(
        min = 1,
        max = 100,
        message = "Age must be realistic"
    ))]
    pub age: Option<i32>,
    pub sexe : Option<Sexe>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe this animal using between 3 and 50 characters"
    ))]
    pub description: Option<String>,
    pub statut: Option<Statut>,
    pub association_id: Option<i32>,
    pub famille_id: Option<Option<i32>>,
    pub espece_id: Option<i32>,
    pub animal_tags: Option<Option<Vec<i32>>>,
}
 */
pub async fn get_animals(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = AnimalRepository::new(db.get_ref());

    let animals = repo
        .find_all()
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(animals))
}

pub async fn get_animal(
    db: web::Data<DbConn>,
    path: web::Path<i32>
) -> Result<HttpResponse, CustomError> {
    let animal_id = path.into_inner();
    let repo = AnimalRepository::new(db.get_ref());

    let animal = repo
        .find_by_id(animal_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    match animal {
        Some(animal) => Ok(HttpResponse::Ok().json(animal)),
        None => Err(CustomError::NotFound),
    }
}

pub async fn get_requests(
    db: web::Data<DbConn>,
    path: web::Path<i32>
) -> Result<HttpResponse, Error> {
    let animal_id = path.into_inner();
    let repo = DemandeRepository::new(db.get_ref());

    let requests = repo
        .find_requests(animal_id)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    Ok(HttpResponse::Ok().json(requests))
}

pub async fn request_animal(
    db: web::Data<DbConn>,
    path: web::Path<i32>
) -> Result<HttpResponse, CustomError> {
    let animal_id = path.into_inner();

    //TODO REMOVE HARDCODED */
    let foster_id = 1;

    info!(
        "Attempting to create request for animal with ID: {}",
        animal_id
    );

    let start_date = Local::now().naive_local().date();
    let six_months = Duration::days(180);
    let end_date = Local::now().naive_local().date() + six_months;

    let request_model = DemandeActiveModel {
        famille_id: Set(foster_id),
        animal_id: Set(animal_id),
        statut_demande: Set(StatutDemande::EnAttente),
        date_debut: Set(start_date),
        date_fin: Set(end_date),
        ..Default::default()
    };

    let repo= DemandeRepository::new(db.get_ref());

    if let Some(_) = repo
        .find_existing(animal_id, foster_id)
        .await
        .map_err(|_e| CustomError::InternalError)?
    {
        return Err(CustomError::AlreadyRequested);
    }

    let new_request = repo
        .create(request_model)
        .await
        .map_err(|_e| CustomError::NotFound)?;

    info!("Request for Animal with ID: {} created with ID: {}", animal_id, new_request.id);

    #[derive(Serialize)]
    struct RequestResponse {
        message: String,
    }

    let request_response = RequestResponse {
        message : "Votre demande a bien été prise en compte !".to_string()
    };
    
    Ok(HttpResponse::Created().json(request_response))
}

pub async fn create_animal(
    db: web::Data<DbConn>,
    json_animal: web::Json<AnimalCreate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_animal)?;

    //TODO REMOVE HARDCODED */
    let shelter_id = 1;

    info!(
        "Attempting to create animal with name: {}",
        json_animal.nom_animal
    );

    let repo = AnimalRepository::new(db.get_ref());

    let animal = json_animal.into_inner();

    let animal_model = AnimalActiveModel {
        nom: Set(animal.nom_animal),
        race: Set(animal.race_animal),
        couleur: Set(animal.couleur_animal),
        age: Set(animal.age_animal),
        sexe : Set(animal.sexe_animal),
        description: Set(animal.description_animal),
        statut: Set(Statut::EnRefuge),
        association_id: Set(shelter_id),
        espece_id: Set(animal.espece_animal),
        ..Default::default()
    };

    let created_animal = repo
        .create(animal_model)
        .await
        .map_err(|_e| CustomError::CreationError)?;

    info!("Animal created with ID: {}", created_animal.id);

    for tag in animal.tags {
        let animal_tag_model = AnimalTagActiveModel {
            animal_id: Set(created_animal.id),
            tag_id: Set(tag),
        };

        AnimalTagRepository::new(db.get_ref())
            .create(animal_tag_model)
            .await
            .map_err(|_e| CustomError::CreationError)?;
    }

    Ok(HttpResponse::Created().json(created_animal))
}
/* pub async fn update_animal(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
    json_animal: web::Json<AnimalUpdate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_animal)?;

    let animal_id = path.into_inner();

    info!("Attempting to update animal with ID: {}", animal_id);

    let repo = AnimalRepository::new(db.get_ref());

    let animal_data = repo
        .find_by_id(animal_id)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    match animal_data {
        Some(animal_data) => {
            let mut animal_active_model: AnimalActiveModelEx = animal_data.into();

            let animal = json_animal.into_inner();

            if let Some(nom) = animal.nom {
                animal_active_model.nom = Set(nom);
            }
            if let Some(race) = animal.race {
                animal_active_model.race = Set(race);
            }
            if let Some(couleur) = animal.couleur {
                animal_active_model.couleur = Set(couleur);
            }
            if let Some(age) = animal.age {
                animal_active_model.age = Set(age);
            }
            if let Some(sexe) = animal.sexe {
                animal_active_model.sexe = Set(sexe);
            }
            if let Some(description) = animal.description {
                animal_active_model.description = Set(description);
            }
            if let Some(statut) = animal.statut {
                animal_active_model.statut = Set(statut);
            }
            if let Some(association_id) = animal.association_id {
                animal_active_model.association_id = Set(association_id);
            }
            if let Some(famille_id) = animal.famille_id {
                animal_active_model.famille_id = Set(famille_id);
            }
            if let Some(espece_id) = animal.espece_id {
                animal_active_model.espece_id = Set(espece_id);
            }

            let updated_animal = repo
                .update(animal_active_model)
                .await
                .map_err(|e| ErrorInternalServerError(format!("Failed to update animal: {}", e)))?;

            info!("Animal with ID {} updated", animal_id);
            Ok(HttpResponse::Ok().json(updated_animal))
        }
        None => Err(ErrorNotFound(format!("Animal with ID {} not found", animal_id))),
    }
}

pub async fn delete_animal(
    db: web::Data<DbConn>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let animal_id = path.into_inner();
    let repo = AnimalRepository::new(db.get_ref());

    info!("Attempting to delete animal with ID: {}", animal_id);

    let animal = repo
        .find_by_id(animal_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Database error: {}", e)))?;
    if animal.is_none() {
        return Err(ErrorNotFound(format!("Animal with ID {} not found", animal_id)));
    }

    let delete_result = repo
        .delete(animal_id)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to delete animal: {}", e)))?;

    if delete_result.rows_affected > 0 {
        info!("Animal with ID {} successfully deleted", animal_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        warn!("Animal with ID {} was not deleted (0 rows affected)", animal_id);
        Err(ErrorInternalServerError(
            "Failed to delete animal (0 rows affected)",
        ))
    }
} */