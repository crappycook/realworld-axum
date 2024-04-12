use axum::Router;

use crate::server::AppState;

mod common;

pub fn init_router(state: AppState) -> Router {
    let router = Router::new().nest("/api", common::register_router());
    router.with_state(state)
    // router
}
