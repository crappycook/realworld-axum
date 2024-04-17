use sea_orm::entity::prelude::*;

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

pub async fn search(
    db: DatabaseConnection,
    name: Option<String>,
    club: Option<String>,
) -> Result<Vec<player::Model>, sea_orm::DbErr> {
    let mut session = player::Entity::find();
    if let Some(name) = name {
        session = session.filter(player::Column::Name.eq(name));
    }
    if let Some(club) = club {
        session = session.filter(player::Column::Club.eq(club));
    }
    session.all(&db).await
}
