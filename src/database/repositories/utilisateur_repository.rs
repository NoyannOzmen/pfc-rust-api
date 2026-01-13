use crate::database::models::{AssociationEntity, FamilleEntity, UtilisateurActiveModel, UtilisateurActiveModelEx, UtilisateurEntity, UtilisateurModel, UtilisateurModelEx};
use sea_orm::{DeleteResult, EntityLoaderTrait};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
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

    pub async fn find_by_id(&self, id: i32) -> Result<Option<UtilisateurModelEx>, DbErr> {
        let user = UtilisateurEntity::load()
            .with(AssociationEntity)
            .with(FamilleEntity)
            .filter_by_id(id)
            .one(self.db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UtilisateurModelEx>, DbErr> {
        let user = UtilisateurEntity::load()
            .with(AssociationEntity)
            .with(FamilleEntity)
            .filter_by_email(email)
            .one(self.db)
            .await?;

        Ok(user)
    }

    pub async fn create(&self, model: UtilisateurActiveModel) -> Result<UtilisateurModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: UtilisateurActiveModelEx) -> Result<UtilisateurModelEx, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        UtilisateurEntity::delete_by_id(id).exec(self.db).await
    }
}