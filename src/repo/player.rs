use sea_orm::entity::prelude::*;

use crate::database::player;

#[tracing::instrument]
pub async fn create(
    db: DatabaseConnection,
    player: player::ActiveModel,
) -> Result<player::Model, sea_orm::DbErr> {
    player.insert(&db).await
}

#[tracing::instrument]
pub async fn get_by_id(
    db: DatabaseConnection,
    id: u64,
) -> Result<Option<player::Model>, sea_orm::DbErr> {
    player::Entity::find_by_id(id).one(&db).await
}

#[tracing::instrument]
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

#[tracing::instrument]
pub async fn update(
    db: DatabaseConnection,
    id: u64,
    p: player::ActiveModel,
) -> Result<sea_orm::UpdateResult, DbErr> {
    player::Entity::update_many()
        .set(p)
        .filter(player::Column::Id.eq(id))
        .exec(&db)
        .await
}

#[tracing::instrument]
pub async fn delete(db: DatabaseConnection, id: u64) -> Result<sea_orm::DeleteResult, DbErr> {
    player::Entity::delete_by_id(id).exec(&db).await
}
