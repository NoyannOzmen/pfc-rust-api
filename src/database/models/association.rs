use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "association")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub nom: String,
    #[sea_orm(column_type = "Text")]
    pub responsable: String,
    #[sea_orm(column_type = "Text")]
    pub rue: String,
    #[sea_orm(column_type = "Text")]
    pub commune: String,
    #[sea_orm(column_type = "Text")]
    pub code_postal: String,
    #[sea_orm(column_type = "Text")]
    pub pays: String,
    #[sea_orm(column_type = "Text")]
    pub siret: String,
    #[sea_orm(column_type = "Text")]
    pub telephone: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub site: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(unique)]
    pub utilisateur_id: i32,
    #[sea_orm(has_many)]
    pub animals: HasMany<super::animal::Entity>,
    #[sea_orm(has_many)]
    pub media: HasMany<super::media::Entity>,
    #[sea_orm(
        belongs_to,
        from = "utilisateur_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    pub utilisateur: HasOne<super::utilisateur::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
