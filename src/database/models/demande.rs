use super::sea_orm_active_enums::StatutDemande;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "demande")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub famille_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub animal_id: i32,
    pub statut_demande: StatutDemande,
    pub date_debut: Date,
    pub date_fin: Date,
    #[sea_orm(
        belongs_to,
        from = "animal_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    pub animal: HasOne<super::animal::Entity>,
    #[sea_orm(
        belongs_to,
        from = "famille_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    pub famille: HasOne<super::famille::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
