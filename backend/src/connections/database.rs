use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::configs;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub fn get_database_connection() -> DatabaseConnection {
    DB.get().unwrap().to_owned()
}

pub(super) async fn init_connection() {
    let db = Database::connect(configs::get_database_config().clone())
        .await
        .expect("Failed to connect on database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations on database");

    DB.set(db).unwrap()
}
