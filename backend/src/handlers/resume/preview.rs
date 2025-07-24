use std::path::PathBuf;

use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use sea_orm::{ColumnTrait, DerivePartialModel, EntityTrait, FromQueryResult, QueryFilter};
use serde_json::json;
use tokio::fs;

use crate::{entities::resume, state::AppState};

#[derive(FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "resume::Entity")]
struct Resume {
    attachment: String,
}

pub async fn preview(
    State(state): State<AppState>,
    Path(resume_id): Path<u32>,
) -> Result<Response, impl IntoResponse> {
    let db = &*state.db_conn;

    let Ok(resume_option) = resume::Entity::find_by_id(resume_id)
        .filter(resume::Column::Attachment.is_not_null())
        .into_partial_model::<Resume>()
        .one(db)
        .await
    else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao procurar usuário"
            })),
        ));
    };

    let Some(resume) = resume_option else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário não encontrado"
            })),
        ));
    };

    let file_path = PathBuf::from(format!("uploads/resume/{}", resume.attachment));

    if fs::metadata(&file_path).await.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao validar usuário"
            })),
        ));
    }

    let Ok(file_bytes) = fs::read(&file_path).await else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Falha ao ler currículo"
            })),
        ));
    };
    let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

    Ok(Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(file_bytes))
        .unwrap())
}
