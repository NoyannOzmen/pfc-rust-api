use crate::database::models::{TagActiveModel, TagEntity, TagModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
};

pub struct TagRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TagRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<TagModel>, DbErr> {
        TagEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<TagModel>, DbErr> {
        TagEntity::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, model: TagActiveModel) -> Result<TagModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: TagActiveModel) -> Result<TagModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        TagEntity::delete_by_id(id).exec(self.db).await
    }
}