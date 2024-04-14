use sea_orm::DatabaseConnection;

use crate::{config::AppConfig, database};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Self {
        let db = database::init_conn_from_config(&config.db).await;
        Self { db }
    }
}
