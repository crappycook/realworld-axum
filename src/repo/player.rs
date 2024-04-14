use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

use crate::database::player;

pub async fn create(
    db: DatabaseConnection,
    player: player::ActiveModel,
) -> Result<player::Model, sea_orm::DbErr> {
    player.insert(&db).await
}

pub async fn get_by_id(
    db: DatabaseConnection,
    id: u64,
) -> Result<Option<player::Model>, sea_orm::DbErr> {
    player::Entity::find_by_id(id).one(&db).await
}
