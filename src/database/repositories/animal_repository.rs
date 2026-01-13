use crate::database::models::{AnimalActiveModel, AnimalActiveModelEx, AnimalEntity, AnimalModel, AnimalModelEx, AssociationEntity, EspeceEntity, FamilleEntity, MediaEntity, TagEntity};
use sea_orm::{DeleteResult, EntityLoaderTrait};
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

    //* Shelter Animals have no picture */
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