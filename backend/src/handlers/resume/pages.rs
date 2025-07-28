use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{EntityTrait, PaginatorTrait};
use serde_json::json;

use crate::{entities::resume, jwt::JwtClaims, state::AppState};

pub async fn pages(State(state): State<AppState>, _: JwtClaims) -> impl IntoResponse {
    let db = &*state.db_conn;

    let Ok(mut pages) = resume::Entity::find().count(db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao procurar usuários"
            })),
        );
    };

    if pages != 0 {
        pages -= 1
    }

    (
        StatusCode::OK,
        Json(json!({
            "pages": pages / 10 + 1
        })),
    )
}
