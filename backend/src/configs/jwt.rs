use std::{env, sync::OnceLock};

#[derive(Clone)]
pub struct JwtOptions {
    pub secret: String,
    pub expiration: usize,
}

static JWT_OPTIONS: OnceLock<JwtOptions> = OnceLock::new();

pub fn get_jwt_config() -> &'static JwtOptions {
    JWT_OPTIONS.get_or_init(|| {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET not found on .env");
        let expiration = env::var("JWT_EXPIRATION")
            .expect("JWT_EXPIRATION not found on .env")
            .parse()
            .expect("JWT_EXPIRATION needs be a number");

        JwtOptions { secret, expiration }
    })
}
