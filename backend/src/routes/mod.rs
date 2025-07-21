use axum::Router;

use crate::state::AppState;

mod login;
mod resume;
mod sector;

pub fn configure_routes() -> Router<AppState> {
    Router::new()
        .nest("/resume", resume::configure_routes())
        .nest("/user", login::configure_routes())
        .nest("/sector", sector::configure_routes())
}
