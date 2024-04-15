use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    database::player::Model,
    dto::player::{AddPlayerReq, AddPlayerResp, PlayerResp},
    repo,
    server::AppState,
};

pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<AddPlayerReq>,
) -> (StatusCode, Json<AddPlayerResp>) {
    let p = Model {
        name: req.name,
        club: req.club,
        ..Default::default()
    };
    let result = repo::player::create(state.db, p.into()).await;

    match result {
        Ok(_) => {
            let rsp = AddPlayerResp { success: true };
            (StatusCode::OK, Json(rsp))
        }
        Err(e) => {
            let rsp = AddPlayerResp { success: false };
            tracing::error!("add player error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(rsp))
        }
    }
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> (StatusCode, Json<Option<PlayerResp>>) {
    let result = repo::player::get_by_id(state.db, id).await;
    match result {
        Ok(player) => {
            if let Some(p) = player {
                (
                    StatusCode::OK,
                    Json(Some(PlayerResp {
                        id,
                        name: p.name,
                        club: p.club,
                    })),
                )
            } else {
                (StatusCode::OK, Json(None))
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
