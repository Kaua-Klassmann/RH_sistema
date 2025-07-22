use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveValue::Set, DerivePartialModel, EntityTrait, FromQueryResult};
use serde_json::json;
use tokio::{fs, io::AsyncWriteExt};

use crate::{entities::resume, jwt::JwtClaims, state::AppState};

#[derive(FromQueryResult, DerivePartialModel)]
#[sea_orm(entity = "resume::Entity")]
struct PartialResume {
    name: String,
    cpf: String,
    attachment: Option<String>,
}

pub async fn upload(
    State(state): State<AppState>,
    _: JwtClaims,
    Path(resume_id): Path<u32>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let db = &*state.db_conn;

    while let Ok(Some(mut field)) = multipart.next_field().await {
        if field.name().is_none() || field.name().unwrap() != "resume" {
            continue;
        }

        if field.file_name().is_none() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Currículo inválido"
                })),
            );
        }

        let Ok(resume_option) = resume::Entity::find_by_id(resume_id)
            .into_partial_model::<PartialResume>()
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

        let Some(resume) = resume_option else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Usuário não encontrado"
                })),
            );
        };

        if resume.attachment.is_some() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Usuário já possui currículo"
                })),
            );
        }

        let filename = field.file_name().map(|f| f.to_owned()).unwrap();

        let Some(extension) = std::path::Path::new(&filename)
            .extension()
            .and_then(|f| f.to_str())
        else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Falha ao salvar currículo"
                })),
            );
        };

        let mut filename = resume.name.replace(" ", "_").to_lowercase();
        filename.push_str(&format!("-{}.{}", resume.cpf, extension));

        let mut file = fs::File::create(format!("./uploads/resume/{}", filename))
            .await
            .unwrap();

        while let Ok(Some(chunk)) = field.chunk().await {
            let write_result = file.write_all(&chunk).await;

            if write_result.is_err() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Falha ao salvar currículo"
                    })),
                );
            }
        }

        let resume = resume::ActiveModel {
            id: Set(resume_id),
            attachment: Set(Some(filename)),
            ..Default::default()
        };

        if resume::Entity::update(resume).exec(db).await.is_err() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Falha ao salvar currículo"
                })),
            );
        }

        return (StatusCode::OK, Json(json!({})));
    }

    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(json!({
            "error": "Estrutura inválida"
        })),
    )
}
