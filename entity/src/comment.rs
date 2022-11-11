//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub url: String,
    pub body: String,
    pub issue_url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub owner_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::owner::Entity",
        from = "Column::OwnerId",
        to = "super::owner::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Owner,
}

impl Related<super::owner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
