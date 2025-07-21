use axum::{Router, routing::post};

use crate::{handlers, state::AppState};

pub(super) fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::resume::create))
        .route("/{resume_id}/edit", post(handlers::resume::edit))
        .route("/{resume_id}/upload", post(handlers::resume::upload))
}
