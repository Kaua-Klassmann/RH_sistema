use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Local;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, FromQueryResult, Order, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, prelude::DateTime,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    entities::{resume, sector},
    jwt::JwtClaims,
    state::AppState,
};

const RESUMES_PER_PAGE: u16 = 10;

#[derive(Deserialize)]
pub struct Payload {
    search: String,
    id_sector: u32,
    uninterviewed: bool,
}

#[derive(Serialize, FromQueryResult)]
struct Resume {
    id: u32,
    name: String,
    cpf: String,
    hired: bool,
    sector: String,
    interview_date: Option<DateTime>,
}

pub async fn list(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(page): Path<u16>,
    Json(payload): Json<Payload>,
) -> impl IntoResponse {
    let db = &*state.db_conn;

    if page == 0 {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Estrutura inválida"
            })),
        );
    }

    let mut condition = Condition::all().add(resume::Column::Attachment.is_not_null());
    let mut order_col = resume::Column::Id;
    let mut order_ord = Order::Desc;

    if !payload.search.is_empty() {
        if payload.search.chars().all(|c| c.is_ascii_digit()) {
            condition = condition.add(resume::Column::Cpf.like(&format!("{}%", payload.search)));
            order_col = resume::Column::Cpf;
        } else {
            condition = condition.add(resume::Column::Name.like(&format!("{}%", payload.search)));
            order_col = resume::Column::Name;
        }

        order_ord = Order::Asc;
    }

    if payload.id_sector != 0 {
        condition = condition.add(resume::Column::IdSector.eq(payload.id_sector));
    }

    if payload.uninterviewed {
        condition = condition.add(resume::Column::InterviewDate.gte(Local::now().naive_local()));
        order_col = resume::Column::InterviewDate;
        order_ord = Order::Asc;
    }

    let Ok(resumes) = resume::Entity::find()
        .columns([
            resume::Column::Id,
            resume::Column::Name,
            resume::Column::Cpf,
            resume::Column::Hired,
            resume::Column::InterviewDate,
        ])
        .column_as(sector::Column::Name, "sector")
        .join(sea_orm::JoinType::InnerJoin, resume::Relation::Sector.def())
        .filter(condition)
        .order_by(order_col, order_ord)
        .limit(Some(RESUMES_PER_PAGE as u64))
        .offset(Some((page as u64 - 1) * RESUMES_PER_PAGE as u64))
        .into_model::<Resume>()
        .all(db)
        .await
    else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao procurar usuários"
            })),
        );
    };

    (
        StatusCode::OK,
        Json(json!({
            "resumes": resumes
        })),
    )
}
