use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Days, Utc};
use sea_orm::{ActiveValue::Set, EntityTrait, prelude::DateTime};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{entities::resume, jwt::JwtClaims, state::AppState};

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(length(min = 3))]
    name: String,
    #[validate(length(min = 11, max = 11))]
    cpf: String,
    #[validate(length(min = 11, max = 11))]
    phone: String,
    #[validate(range(min = 1))]
    id_sector: u32,
    hired: bool,
    interview_date: Option<DateTime>,
    observation: String,
}

pub async fn edit(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(resume_id): Path<u32>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    if payload.validate().is_err()
        || !payload.cpf.chars().all(|c| c.is_ascii_digit())
        || !payload.phone.chars().all(|c| c.is_ascii_digit())
    {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Estrutura inválida"
            })),
        );
    }

    if payload.interview_date.is_some() {
        let now = Utc::now()
            .naive_utc()
            .checked_sub_days(Days::new(1))
            .unwrap();

        if now.gt(&payload.interview_date.unwrap()) {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Falha ao atualizar com data já passada"
                })),
            );
        }
    }

    let db = &*state.db_conn;

    let Ok(resume_option) = resume::Entity::find_by_id(resume_id).one(db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao procurar usuário"
            })),
        );
    };

    if resume_option.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário não cadastrado no sistema"
            })),
        );
    };

    let resume = resume::ActiveModel {
        id: Set(resume_id),
        name: Set(payload.name),
        cpf: Set(payload.cpf),
        phone: Set(payload.phone),
        hired: Set(payload.hired),
        id_sector: Set(payload.id_sector),
        interview_date: Set(payload.interview_date),
        observation: Set(payload.observation),
        ..Default::default()
    };

    if resume::Entity::update(resume).exec(db).await.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao atualizar usuário"
            })),
        );
    };

    (StatusCode::OK, Json(json!({})))
}
