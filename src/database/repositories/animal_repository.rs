use crate::database::models::animal::{self};
use crate::database::models::{AnimalActiveModel, AnimalActiveModelEx, AnimalEntity, AnimalModel, AnimalModelEx, AssociationEntity, DemandeEntity, EspeceEntity, FamilleEntity, MediaEntity, TagEntity};
use crate::database::models::sea_orm_active_enums::Statut::*;
use sea_orm::{ColumnTrait, DeleteResult, EntityLoaderTrait, QueryFilter};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct AnimalRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AnimalRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<AnimalModelEx>, DbErr> {
        let animals= AnimalEntity::load()
            .with((AssociationEntity, MediaEntity))
            .with(MediaEntity)
            .with(FamilleEntity)
            .with(EspeceEntity)
            .with(TagEntity)
            .all(self.db)
            .await?;

        Ok(animals)
    }

    pub async fn find_fostered(&self, id: i32) -> Result<Vec<AnimalModelEx>, DbErr> {
        let animals= AnimalEntity::load()
            .with((AssociationEntity, MediaEntity))
            .with(MediaEntity)
            .with(FamilleEntity)
            .with(EspeceEntity)
            .with(TagEntity)
            .filter(animal::COLUMN.association_id.eq(id))
            .filter(animal::COLUMN.statut.eq(Accueilli))
            .all(self.db)
            .await?;

        Ok(animals)
    }

    pub async fn find_requested(&self, id: i32) -> Result<Vec<AnimalModelEx>, DbErr> {
        let mut animals= AnimalEntity::load()
            .with((AssociationEntity, MediaEntity))
            .with(MediaEntity)
            .with(FamilleEntity)
            .with(EspeceEntity)
            .with(TagEntity)
            .with((DemandeEntity, FamilleEntity))
            .filter(animal::COLUMN.association_id.eq(id))
            .all(self.db)
            .await?;

        animals = animals.iter().filter(|&animals| !animals.demandes.is_empty()).cloned().collect();

        Ok(animals)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<AnimalModelEx>, DbErr> {
        let animal = AnimalEntity::load()
            .with((AssociationEntity, MediaEntity))
            .with((AssociationEntity, AnimalEntity))
            .with(MediaEntity)
            .with(FamilleEntity)
            .with(EspeceEntity)
            .with(TagEntity)
            .filter_by_id(id)
            .one(self.db)
            .await?;

        Ok(animal)
    }

    pub async fn create(&self, model: AnimalActiveModel) -> Result<AnimalModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: AnimalActiveModelEx) -> Result<AnimalModelEx, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        AnimalEntity::delete_by_id(id).exec(self.db).await
    }
}