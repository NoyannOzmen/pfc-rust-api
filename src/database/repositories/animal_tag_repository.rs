use crate::database::models::{AnimalTagActiveModel, AnimalTagModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

pub struct AnimalTagRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> AnimalTagRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, model: AnimalTagActiveModel) -> Result<AnimalTagModel, DbErr> {
        model.insert(self.db).await
    }
}