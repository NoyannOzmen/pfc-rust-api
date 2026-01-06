use crate::database::models::{AnimalActiveModel, AnimalEntity, AnimalModel};
use sea_orm::DeleteResult;
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

    pub async fn find_all(&self) -> Result<Vec<AnimalModel>, DbErr> {
        AnimalEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<AnimalModel>, DbErr> {
        AnimalEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: AnimalActiveModel) -> Result<AnimalModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: AnimalActiveModel) -> Result<AnimalModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        AnimalEntity::delete_by_id(id).exec(self.db).await
    }
}