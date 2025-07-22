use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QuerySelect,
    RelationTrait, prelude::DateTime,
};
use serde::Serialize;
use serde_json::json;

use crate::{
    configs::get_app_config,
    entities::{resume, sector},
    jwt::JwtClaims,
    state::AppState,
};

#[derive(Serialize, FromQueryResult)]
struct Resume {
    name: String,
    cpf: String,
    phone: String,
    sector: String,
    interview_date: Option<DateTime>,
    hired: bool,
    attachment: Option<String>,
    observation: String,
}

pub async fn view(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(resume_id): Path<u32>,
) -> impl IntoResponse {
    let db = &*state.db_conn;

    let Ok(resume_exists) = resume::Entity::find_by_id(resume_id)
        .filter(resume::Column::Attachment.is_not_null())
        .count(db)
        .await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao validar usuário"
            })),
        );
    };

    if resume_exists == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário não encontrado"
            })),
        );
    }

    let Ok(resume_option) = resume::Entity::find_by_id(resume_id)
        .columns([
            resume::Column::Name,
            resume::Column::Cpf,
            resume::Column::Phone,
            resume::Column::InterviewDate,
            resume::Column::Hired,
            resume::Column::Attachment,
            resume::Column::Observation,
        ])
        .column_as(sector::Column::Name, "sector")
        .join(sea_orm::JoinType::InnerJoin, resume::Relation::Sector.def())
        .into_model::<Resume>()
        .one(db)
        .await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao procurar usuário"
            })),
        );
    };

    let Some(mut resume) = resume_option else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário não encontrado"
            })),
        );
    };

    resume.attachment = Some(format!(
        "{}/resume/{}",
        get_app_config().backend_url,
        resume.attachment.unwrap()
    ));

    let resume = serde_json::to_value(resume).unwrap();

    (StatusCode::OK, Json(resume))
}
