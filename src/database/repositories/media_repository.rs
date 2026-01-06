use crate::database::models::{MediaActiveModel, MediaEntity, MediaModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct MediaRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> MediaRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<MediaModel>, DbErr> {
        MediaEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<MediaModel>, DbErr> {
        MediaEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: MediaActiveModel) -> Result<MediaModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: MediaActiveModel) -> Result<MediaModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        MediaEntity::delete_by_id(id).exec(self.db).await
    }
}