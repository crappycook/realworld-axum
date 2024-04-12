use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::common, server::AppState};

pub fn register_router() -> Router<AppState> {
    Router::new().nest(
        "/common",
        Router::new()
            .route("/system_time", get(common::system_time))
            .route("/echo", post(common::echo)),
    )
}
