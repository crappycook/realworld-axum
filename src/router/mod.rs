use axum::{extract::MatchedPath, http::Request, Router};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::server::AppState;

mod common;
mod player;

pub fn init_router(state: AppState) -> Router {
    let mut api_router = Router::new();
    api_router = api_router
        .merge(common::register_router())
        .merge(player::register_router());

    let router =
        Router::new()
            .nest("/api", api_router)
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                }),
            );
    router.with_state(state)
}
