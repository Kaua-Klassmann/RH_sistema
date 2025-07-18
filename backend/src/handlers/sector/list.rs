use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{EntityTrait, FromQueryResult};
use serde::Serialize;
use serde_json::json;

use crate::{entities::sector, state::AppState};

#[derive(Serialize, FromQueryResult)]
struct Sector {
    id: u32,
    name: String,
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let db = &*state.db_conn;

    let Ok(sectors) = sector::Entity::find().into_model::<Sector>().all(db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao produrar setores"
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "setores": sectors
        })),
    )
}
