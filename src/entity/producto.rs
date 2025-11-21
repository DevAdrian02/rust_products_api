use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "producto")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub nombre: String,
    pub precio: i32,
    pub stock: i32,

    #[sea_orm(default_value = "now()")]
    pub creado_el: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
