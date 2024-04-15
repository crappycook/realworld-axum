use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u64,
    pub name: String,
    pub club: String,
    #[sea_orm(column_type = "DateTime")]
    pub created_at: chrono::NaiveDateTime,
    #[sea_orm(column_type = "DateTime")]
    pub updated_at: chrono::NaiveDateTime,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            club: String::new(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
