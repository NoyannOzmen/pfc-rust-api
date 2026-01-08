use super::sea_orm_active_enums::Sexe;
use super::sea_orm_active_enums::Statut;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "animal")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub nom: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub race: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub couleur: String,
    pub age: i32,
    pub sexe: Sexe,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub statut: Statut,
    pub association_id: i32,
    pub famille_id: Option<i32>,
    pub espece_id: i32,
    #[sea_orm(has_many)]
    pub tags: HasMany<super::animal_tag::Entity>,
    #[sea_orm(
        belongs_to,
        from = "association_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    pub association: HasOne<super::association::Entity>,
    #[sea_orm(has_many)]
    pub demandes: HasMany<super::demande::Entity>,
    #[sea_orm(
        belongs_to,
        from = "espece_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    pub espece: HasOne<super::espece::Entity>,
    #[sea_orm(
        belongs_to,
        from = "famille_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub famille: HasOne<super::famille::Entity>,
    #[sea_orm(has_many)]
    pub images_animal: HasMany<super::media::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
