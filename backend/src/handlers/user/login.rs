use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{configs, jwt};

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(length(min = 3))]
    user: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn login(Json(payload): Json<Payload>) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": "Estrutura inválida"
            })),
        );
    }

    let user_config = configs::get_app_config();

    if payload.user != user_config.user {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Usuário inválido"
            })),
        );
    }

    if payload.password != user_config.password {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Senha inválida"
            })),
        );
    }

    let token = jwt::JwtClaims::new().gen_token();

    (
        StatusCode::OK,
        Json(json!({
            "token": token
        })),
    )
}
