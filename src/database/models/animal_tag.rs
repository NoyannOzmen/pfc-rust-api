use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, serde::Serialize  )]
#[sea_orm(table_name = "animal_tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub animal_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i32,
    #[sea_orm(
        belongs_to,
        from = "animal_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    pub animal: Option<super::animal::Entity>,
    #[sea_orm(
        belongs_to,
        from = "tag_id",
        to = "id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    pub tag: Option<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
