use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    Json, RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::configs;

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    exp: usize,
}

impl JwtClaims {
    pub fn new() -> Self {
        let expiration_time = Self::get_expiration_time();

        JwtClaims {
            exp: expiration_time,
        }
    }

    fn get_expiration_time() -> usize {
        let jwt_options = configs::get_jwt_config();

        let now = SystemTime::now();
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

        let expiration_time = jwt_options.expiration;

        duration_since_epoch.as_secs() as usize + expiration_time
    }

    pub fn gen_token(&self) -> String {
        let jwt_options = configs::get_jwt_config();

        let secret = jwt_options.secret.as_bytes();

        encode(&Header::default(), &self, &EncodingKey::from_secret(secret)).unwrap()
    }

    fn parse_token(token: String) -> Result<JwtClaims, impl IntoResponse> {
        let jwt_options = configs::get_jwt_config();

        let secret = jwt_options.secret.as_bytes();

        match decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        ) {
            Ok(claim) => Ok(claim.claims),
            Err(_) => Err(Errors::InvalidToken),
        }
    }
}

pub enum Errors {
    InvalidToken,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Token inválido".to_string()
            })),
        )
            .into_response()
    }
}

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = Errors;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            return Err(Errors::InvalidToken);
        };

        JwtClaims::parse_token(bearer.token().to_string()).map_err(|_| Errors::InvalidToken)
    }
}
