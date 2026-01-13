use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub nom: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
}

impl ActiveModelBehavior for ActiveModel {}
