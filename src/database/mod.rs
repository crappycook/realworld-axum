pub mod player;

use crate::config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn init_conn_from_config(cfg: &DatabaseConfig) -> DatabaseConnection {
    let mut opts = ConnectOptions::new(cfg.get_url());
    opts.max_connections(cfg.max_connections);
    opts.connect_timeout(Duration::from_secs(8));
    opts.sqlx_logging(true);

    Database::connect(opts)
        .await
        .expect("Database connection failed")
}
