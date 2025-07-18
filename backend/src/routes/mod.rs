use axum::Router;

use crate::state::AppState;

mod login;
mod sector;

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", login::configure_routes())
        .nest("/sector", sector::configure_routes())
}
