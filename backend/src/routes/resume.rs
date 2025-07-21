use axum::{Router, routing::post};

use crate::{handlers, state::AppState};

pub(super) fn configure_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handlers::resume::create))
        .route(
            "/edit/{resume_id}/interview_date",
            post(handlers::resume::edit_interview_date),
        )
}
