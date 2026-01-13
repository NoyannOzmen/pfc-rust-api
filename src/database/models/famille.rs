use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "famille")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub prenom: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub nom: String,
    #[sea_orm(column_type = "Text")]
    pub telephone: String,
    #[sea_orm(column_type = "Text")]
    pub rue: String,
    #[sea_orm(column_type = "Text")]
    pub commune: String,
    #[sea_orm(column_type = "Text")]
    pub code_postal: String,
    #[sea_orm(column_type = "Text")]
    pub pays: String,
    #[sea_orm(column_type = "Text")]
    pub hebergement: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub terrain: Option<String>,
    #[sea_orm(unique)]
    pub utilisateur_id: i32,
    #[sea_orm(has_many, via = "demande")]
    pub animals: HasMany<super::animal::Entity>,
    #[sea_orm(has_many)]
    pub demandes: HasMany<super::demande::Entity>,
    #[sea_orm(
        belongs_to,
        from = "utilisateur_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    pub identifiant_famille: HasOne<super::utilisateur::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
