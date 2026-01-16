use crate::database::models::association::{self};
use crate::database::models::{AnimalEntity, AssociationActiveModel, AssociationActiveModelEx, AssociationEntity, AssociationModel, AssociationModelEx, EspeceEntity, MediaEntity};
use sea_orm::{DeleteResult, EntityLoaderTrait, QueryFilter};
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

    pub async fn find_all(&self) -> Result<Vec<AssociationModelEx>, DbErr> {
        let shelters = AssociationEntity::load()
            .with(AnimalEntity)
            .with(MediaEntity)
            .all(self.db)
            .await?;

        Ok(shelters)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<AssociationModelEx>, DbErr> {
        let shelter  = AssociationEntity::load()
            .with((AnimalEntity, MediaEntity))
            .with((AnimalEntity, EspeceEntity))
            .with((AnimalEntity, AssociationEntity))
            .with(MediaEntity)
            .filter_by_id(id)
            .one(self.db)
            .await?;

        Ok(shelter)
    }

    pub async fn find_by_user_id(&self, id: i32) -> Result<Option<AssociationModelEx>, DbErr> {
        let foster = AssociationEntity::load()
            .with(AnimalEntity)
            .filter(association::COLUMN.utilisateur_id.eq(id))
            .one(self.db)
            .await?;

        Ok(foster)
    }

    pub async fn create(&self, model: AssociationActiveModel) -> Result<AssociationModel, DbErr> {
        model.insert(self.db).await
    }

    pub async fn update(&self, model: AssociationActiveModelEx) -> Result<AssociationModelEx, DbErr> {
        model.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        AssociationEntity::delete_by_id(id).exec(self.db).await
    }
}