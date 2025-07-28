use axum::{
    Router,
    routing::{get, patch, post, put},
};

use crate::{handlers, state::AppState};

pub(super) fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::resume::create))
        .route("/list/{page}", get(handlers::resume::list))
        .route("/{resume_id}/view", get(handlers::resume::view))
        .route("/{resume_id}/edit", put(handlers::resume::edit))
        .route("/{resume_id}/hire", patch(handlers::resume::hire))
        .route("/{resume_id}/upload", post(handlers::resume::upload))
        .route("/{resume_id}/preview", get(handlers::resume::preview))
        .route("/{resume_id}/download", get(handlers::resume::download))
}
