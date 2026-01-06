use crate::database::models::{EspeceActiveModel, EspeceEntity, EspeceModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct EspeceRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> EspeceRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<EspeceModel>, DbErr> {
        EspeceEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<EspeceModel>, DbErr> {
        EspeceEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: EspeceActiveModel) -> Result<EspeceModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: EspeceActiveModel) -> Result<EspeceModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        EspeceEntity::delete_by_id(id).exec(self.db).await
    }
}