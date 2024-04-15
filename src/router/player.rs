use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::player, server::AppState};

pub fn register_router() -> Router<AppState> {
    Router::new().nest(
        "/player",
        Router::new()
            .route("/:id", get(player::get))
            .route("/add", post(player::add)),
    )
}
