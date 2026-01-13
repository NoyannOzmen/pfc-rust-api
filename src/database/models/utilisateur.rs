use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "utilisateur")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", unique)]
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub mot_de_passe: String,
    #[sea_orm(has_one)]
    pub refuge: HasOne<super::association::Entity>,
    #[sea_orm(has_one)]
    pub accueillant: HasOne<super::famille::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
