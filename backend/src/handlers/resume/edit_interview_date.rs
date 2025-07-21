use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel, prelude::DateTime,
};
use serde::Deserialize;
use serde_json::json;

use crate::{entities::resume, jwt::JwtClaims, state::AppState};

#[derive(Deserialize)]
pub struct Payload {
    interview_date: DateTime,
}

pub async fn edit_interview_date(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(resume_id): Path<u32>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    let db = &*state.db_conn;

    let Ok(resume_option) = resume::Entity::find_by_id(resume_id).one(db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao validar usuário"
            })),
        );
    };

    let Some(resume_model) = resume_option else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário não encontrado"
            })),
        );
    };

    let mut resume = resume_model.into_active_model();

    resume.interview_date = Set(Some(payload.interview_date));

    let Ok(_) = resume.save(db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao atualizar usuário"
            })),
        );
    };

    (StatusCode::OK, Json(json!({})))
}
