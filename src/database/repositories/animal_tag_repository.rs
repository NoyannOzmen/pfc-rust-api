/* use crate::database::models::{AnimalTagActiveModel, AnimalTagEntity, AnimalTagModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct AnimalTagRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AnimalTagRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<AnimalTagModel>, DbErr> {
        AnimalTagEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<AnimalTagModel>, DbErr> {
        AnimalTagEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: AnimalTagActiveModel) -> Result<AnimalTagModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: AnimalTagActiveModel) -> Result<AnimalTagModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        AnimalTagEntity::delete_by_id(id).exec(self.db).await
    }
} */