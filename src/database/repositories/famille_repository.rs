use crate::database::models::{AnimalEntity, FamilleActiveModel, FamilleActiveModelEx, FamilleEntity, FamilleModel, FamilleModelEx};
use sea_orm::{DeleteResult, EntityLoaderTrait};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct FamilleRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> FamilleRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<FamilleModel>, DbErr> {
        FamilleEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<FamilleModelEx>, DbErr> {
        let foster = FamilleEntity::load()
            .with(AnimalEntity)
            .filter_by_id(id)
            .one(self.db)
            .await?;

        Ok(foster)
    }

    pub async fn create(&self, model: FamilleActiveModel) -> Result<FamilleModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: FamilleActiveModelEx) -> Result<FamilleModelEx, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        FamilleEntity::delete_by_id(id).exec(self.db).await
    }
}