use std::{env, sync::OnceLock};

pub struct AppConfig {
    pub port: u32,
    pub user: String,
    pub password: String,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn get_app_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| {
        let port = env::var("APP_PORT")
            .expect("APP_PORT not found on .env")
            .parse()
            .expect("APP_PORT needs be a number");
        let user = env::var("APP_USER").expect("APP_USER not found on .env");
        let password = env::var("APP_PASSWORD").expect("APP_PASSWORD not found on .env");

        AppConfig {
            port,
            user,
            password,
        }
    })
}
