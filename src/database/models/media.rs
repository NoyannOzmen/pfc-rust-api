use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub url: String,
    pub ordre: Option<i32>,
    pub animal_id: Option<i32>,
    pub association_id: Option<i32>,
    #[sea_orm(
        belongs_to,
        from = "animal_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub animal: HasOne<super::animal::Entity>,
    #[sea_orm(
        belongs_to,
        from = "association_id",
        to = "id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    pub association: HasOne<super::association::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
