use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "espece")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub nom: String,
    #[sea_orm(has_many)]
    pub animals: HasMany<super::animal::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
