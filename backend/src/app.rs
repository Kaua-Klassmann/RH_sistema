use std::sync::Arc;

use axum::Router;

use crate::{connections, routes, state::AppState};

pub async fn create_app() -> Router {
    connections::init_connections().await;

    let db_conn = Arc::new(connections::get_database_connection());

    let state = AppState { db_conn };

    routes::configure_routes().with_state(state)
}
