use crate::setting::SETTINGS;
use sea_orm::Database;
use tracing::debug;

pub async fn setup_db_connection() -> sea_orm::DatabaseConnection {
    debug!("Connecting to database");
    let db = Database::connect(format!(
        "{}/{}",
        SETTINGS.database.uri, SETTINGS.database.name
    ))
    .await
    .map_err(|e| {
        tracing::error!("Failed to connect to database: {}", e);
        e
    })
    .unwrap();

    debug!("Connected to database");

    db
}
