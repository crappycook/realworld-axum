use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::Set;

use crate::{
    constant::LOCAL_TIMEZONE,
    database::player::{ActiveModel, Model},
    dto::player::*,
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

pub async fn search(
    State(state): State<AppState>,
    Query(req): Query<QueryPlayerReq>,
) -> (StatusCode, Json<Vec<PlayerItem>>) {
    let result = repo::player::search(state.db, req.name, req.club).await;
    match result {
        Ok(list) => {
            let rsp = list
                .into_iter()
                .map(|p| PlayerItem {
                    id: p.id,
                    name: p.name,
                    club: p.club,
                    created_at: p
                        .created_at
                        .and_local_timezone(LOCAL_TIMEZONE)
                        .unwrap()
                        .timestamp(),
                })
                .collect();
            (StatusCode::OK, Json(rsp))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn update(
    State(state): State<AppState>,
    Json(req): Json<UpdatePlayerReq>,
) -> (StatusCode, Json<UpdatePlayerResp>) {
    let p = ActiveModel {
        name: Set(req.name),
        club: Set(req.club),
        updated_at: Set(chrono::Local::now().naive_local()),
        ..Default::default()
    };
    let result = repo::player::update(state.db, req.id, p).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(UpdatePlayerResp { success: true })),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UpdatePlayerResp { success: false }),
        ),
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Json(req): Json<DeletePlayerReq>,
) -> (StatusCode, Json<DeletePlayerResp>) {
    let result = repo::player::delete(state.db, req.id).await;
    match result {
        Ok(_) => (StatusCode::OK, Json(DeletePlayerResp { success: true })),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DeletePlayerResp { success: false }),
        ),
    }
}
