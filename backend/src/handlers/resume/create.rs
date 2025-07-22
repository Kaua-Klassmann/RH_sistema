use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Days, Local};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    prelude::DateTime,
};
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
    interview_date: Option<DateTime>,
    observation: String,
}

pub async fn create(
    State(state): State<AppState>,
    _: JwtClaims,
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
        let now = Local::now()
            .naive_local()
            .checked_sub_days(Days::new(1))
            .unwrap();

        if now.gt(&payload.interview_date.unwrap()) {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Falha ao cadastrar com data já passada"
                })),
            );
        }
    }

    let db = &*state.db_conn;

    let Ok(resume_exists) = resume::Entity::find()
        .filter(resume::Column::Cpf.eq(&payload.cpf))
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

    if resume_exists != 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário já cadastrado no sistema"
            })),
        );
    }

    let resume = resume::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        cpf: Set(payload.cpf),
        phone: Set(payload.phone),
        id_sector: Set(payload.id_sector),
        interview_date: Set(payload.interview_date),
        hired: NotSet,
        attachment: NotSet,
        observation: Set(payload.observation),
    };

    let Ok(create_resume_result) = resume::Entity::insert(resume).exec(db).await else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao cadastrar usuário"
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "id": create_resume_result.last_insert_id
        })),
    )
}
