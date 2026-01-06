use crate::database::models::{DemandeActiveModel, DemandeEntity, DemandeModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct DemandeRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DemandeRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<DemandeModel>, DbErr> {
        DemandeEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<DemandeModel>, DbErr> {
        DemandeEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: DemandeActiveModel) -> Result<DemandeModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: DemandeActiveModel) -> Result<DemandeModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        DemandeEntity::delete_by_id(id).exec(self.db).await
    }
}