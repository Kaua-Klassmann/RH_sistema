use std::sync::Arc;

use axum::{Router, extract::DefaultBodyLimit};

use crate::{connections, routes, state::AppState};

const DEFAULT_BODY_LIMIT: usize = 10 * 1024 * 1024; // 10 MB

pub async fn create_app() -> Router {
    connections::init_connections().await;

    let db_conn = Arc::new(connections::get_database_connection());

    let state = AppState { db_conn };

    routes::configure_routes()
        .layer(DefaultBodyLimit::max(DEFAULT_BODY_LIMIT))
        .with_state(state)
}
