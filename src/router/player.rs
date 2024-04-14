use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{common, player},
    server::AppState,
};

pub fn register_router() -> Router<AppState> {
    Router::new().nest(
        "/player",
        Router::new()
            .route("/get", get(common::system_time))
            .route("/add", post(player::add)),
    )
}
