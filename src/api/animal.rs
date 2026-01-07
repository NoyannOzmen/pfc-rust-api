use actix_web::error::{ErrorInternalServerError, ErrorNotFound, /* ErrorUnprocessableEntity */};
use actix_web::{Error, HttpResponse, web};
use log::{info, warn};
use sea_orm::DbConn;
use validator::Validate;

use serde::{Deserialize, Serialize};

use crate::database::models::AnimalActiveModel;
use crate::database::models::sea_orm_active_enums::{Sexe, Statut};
use crate::database::repositories::AnimalRepository;
use crate::validators::common_validators::{process_json_validation};

use sea_orm::ActiveValue::Set;

pub fn configure_public(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("")
            .get(get_animals)
            .post(create_animal)
        )
        .service(web::resource("/{id}")
            .get(get_animal)
            .put(update_animal)
            .delete(delete_animal)
        );
}

#[derive(Deserialize, Serialize, Validate)]
pub struct AnimalCreate {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub nom: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "This must be between 3 and 50 characters"
    ))]
    pub race: Option<String>,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Colour name must be between 3 and 50 characters"
    ))]
    pub couleur: String,
    #[validate(range(
        min = 1,
        max = 100,
        message = "Age must be realistic"
    ))]
    pub age: i32,
    pub sexe : Sexe,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Please describe this animal using between 3 and 50 characters"
    ))]
    pub description: String,
    pub statut: Statut,
    pub association_id: i32,
    pub famille_id: Option<i32>,
    pub espece_id: i32,
    pub animal_tags: Option<Vec<i32>>,
}

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

pub async fn get_animals(db: web::Data<DbConn>) -> Result<HttpResponse, Error> {
    let repo = AnimalRepository::new(db.get_ref());

    let animals = repo
        .find_all()
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve animals: {}", e)))?;

    Ok(HttpResponse::Ok().json(animals))
}

pub async fn get_animal(db: web::Data<DbConn>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let animal_id = path.into_inner();
    let repo = AnimalRepository::new(db.get_ref());

    let animal = repo
        .find_by_id(animal_id)
        .await
        .map_err(|e| ErrorNotFound(format!("Failed to retrieve animal: {}", e)))?;

    match animal {
        Some(animal) => Ok(HttpResponse::Ok().json(animal)),
        None => Err(ErrorNotFound(format!("animal with ID {} not found", animal_id))),
    }
}

pub async fn create_animal(
    db: web::Data<DbConn>,
    json_animal: web::Json<AnimalCreate>,
) -> Result<HttpResponse, Error> {
    process_json_validation(&json_animal)?;

    info!(
        "Attempting to create animal with name: {}",
        json_animal.nom
    );

    let repo = AnimalRepository::new(db.get_ref());

    let animal = json_animal.into_inner();

    let animal_model = AnimalActiveModel {
        nom: Set(animal.nom),
        race: Set(animal.race),
        couleur: Set(animal.couleur),
        age: Set(animal.age),
        sexe : Set(animal.sexe),
        description: Set(animal.description),
        statut: Set(animal.statut),
        association_id: Set(animal.association_id),
        famille_id: Set(animal.famille_id),
        espece_id: Set(animal.espece_id),
        ..Default::default()
    };

    let created_animal = repo
        .create(animal_model)
        .await
        .map_err(|e| ErrorInternalServerError(format!("Failed to create animal: {}", e)))?;

    info!("Animal created with ID: {}", created_animal.id);
    Ok(HttpResponse::Created().json(created_animal))
}

pub async fn update_animal(
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
            let mut animal_active_model: AnimalActiveModel = animal_data.into();

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
}