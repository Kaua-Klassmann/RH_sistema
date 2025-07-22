use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveValue::Set, DerivePartialModel, EntityTrait, FromQueryResult};
use serde_json::json;

use crate::{entities::resume, jwt::JwtClaims, state::AppState};

#[derive(FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "resume::Entity")]
struct PartialResume {
    hired: bool,
}

pub async fn hire(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(resume_id): Path<u32>,
) -> impl IntoResponse {
    let db = &*state.db_conn;

    let Ok(resume_option) = resume::Entity::find_by_id(resume_id)
        .into_partial_model::<PartialResume>()
        .one(db)
        .await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao validar usuário"
            })),
        );
    };

    let Some(resume) = resume_option else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário não encontrado"
            })),
        );
    };

    let resume_model = resume::ActiveModel {
        id: Set(resume_id),
        hired: Set(!resume.hired),
        ..Default::default()
    };

    if resume::Entity::update(resume_model).exec(db).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao atualizar usuário"
            })),
        );
    }

    (StatusCode::OK, Json(json!({})))
}
