use crate::database::models::{UtilisateurActiveModel, UtilisateurColumn, UtilisateurEntity, UtilisateurModel};
use sea_orm::DeleteResult;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

pub struct UtilisateurRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UtilisateurRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<UtilisateurModel>, DbErr> {
        UtilisateurEntity::find().all(self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<UtilisateurModel>, DbErr> {
        UtilisateurEntity::find_by_id(id).one(self.db).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UtilisateurModel>, DbErr> {
       UtilisateurEntity::find()
            .filter(UtilisateurColumn::Email.eq(email))
            .one(self.db)
            .await
    }

    pub async fn create(&self, model: UtilisateurActiveModel) -> Result<UtilisateurModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: UtilisateurActiveModel) -> Result<UtilisateurModel, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        UtilisateurEntity::delete_by_id(id).exec(self.db).await
    }
}