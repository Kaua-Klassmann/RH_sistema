use axum::{Router, routing::post};

use crate::{handlers, state::AppState};

pub(super) fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::resume::create))
        .route("/edit/{resume_id}", post(handlers::resume::edit))
}
