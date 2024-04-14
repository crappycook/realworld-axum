use axum::{extract::State, http::StatusCode, Json};
use sea_orm::Set;

use crate::{
    database::player::ActiveModel,
    dto::player::{AddPlayerReq, AddPlayerResp},
    repo,
    server::AppState,
};

pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<AddPlayerReq>,
) -> (StatusCode, Json<AddPlayerResp>) {
    let result = repo::player::create(
        state.db,
        ActiveModel {
            name: Set(req.name),
            club: Set(req.club),
            ..Default::default()
        },
    )
    .await;

    match result {
        Ok(_) => {
            let rsp = AddPlayerResp { success: true };
            (StatusCode::OK, Json(rsp))
        }
        Err(e) => {
            let rsp = AddPlayerResp { success: false };
            tracing::error!("add player error: {}", e);
            (StatusCode::OK, Json(rsp))
        }
    }
}
