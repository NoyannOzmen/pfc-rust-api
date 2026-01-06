use crate::database::models::{AssociationActiveModel, AssociationEntity, AssociationModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct AssociationRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AssociationRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<AssociationModel>, DbErr> {
        AssociationEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<AssociationModel>, DbErr> {
        AssociationEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: AssociationActiveModel) -> Result<AssociationModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: AssociationActiveModel) -> Result<AssociationModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        AssociationEntity::delete_by_id(id).exec(self.db).await
    }
}