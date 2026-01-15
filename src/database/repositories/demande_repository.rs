use crate::database::models::{AnimalEntity, AssociationEntity, DemandeActiveModel, DemandeActiveModelEx, DemandeEntity, DemandeModel, DemandeModelEx, EspeceEntity, FamilleEntity, MediaEntity, demande};
use sea_orm::{DeleteResult, EntityLoaderTrait, QueryFilter};
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

    pub async fn find_current_requests(&self) -> Result<Vec<DemandeModelEx>, DbErr> {
        //TODO REMOVE HARDCODED */
        let foster_id = 1;

        let requests = DemandeEntity::load()
            .with(FamilleEntity)
            .with((AnimalEntity, AssociationEntity))
            .filter(sea_orm::ColumnTrait::eq(&demande::Column::FamilleId, foster_id))
            .all(self.db)
            .await?;

        Ok(requests)
    }

    pub async fn find_requests(&self, id: i32) -> Result<Vec<DemandeModelEx>, DbErr> {
        let requests = DemandeEntity::load()
            .with((AnimalEntity, AssociationEntity))
            .with(FamilleEntity)
            .filter(sea_orm::ColumnTrait::eq(&demande::Column::AnimalId, id))
            .all(self.db)
            .await?;

        Ok(requests)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<DemandeModelEx>, DbErr> {
        let request = DemandeEntity::load()
            .with(FamilleEntity)
            .with((AnimalEntity, MediaEntity))
            .with((AnimalEntity, EspeceEntity))
            .filter_by_id(id)
            .one(self.db)
            .await?;

        Ok(request)
    }

    pub async fn create(&self, model: DemandeActiveModel) -> Result<DemandeModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: DemandeActiveModelEx) -> Result<DemandeModelEx, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        DemandeEntity::delete_by_id(id).exec(self.db).await
    }
}