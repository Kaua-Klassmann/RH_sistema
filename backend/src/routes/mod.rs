use axum::Router;

use crate::state::AppState;

mod login;

pub fn configure_routes() -> Router<AppState> {
    Router::new().nest("/user", login::configure_routes())
}
