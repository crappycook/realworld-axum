pub mod player;

use crate::config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn init_conn_from_config(cfg: &DatabaseConfig) -> DatabaseConnection {
    let mut opts = ConnectOptions::new(cfg.get_url());
    opts.max_connections(cfg.max_connections);
    opts.connect_timeout(Duration::from_secs(8));
    opts.sqlx_logging(true);

    // SET sql_mode=(SELECT CONCAT(@@sql_mode, ',PIPES_AS_CONCAT,NO_ENGINE_SUBSTITUTION')),time_zone='+00:00',NAMES …"
    // [WARN] time_zone is default to UTC, `timezone` param in mysql uri cannot be parsed.
    // Source code:
    // sea-orm/src/driver/sqlx-mysql.rs `impl SqlxMySqlConnector` `connect(options: ConnectOptions)`
    // sqlx-mysql/src/options/parse.rs `impl MySqlConnectOptions` `parse_from_url(url: &Url)`
    Database::connect(opts)
        .await
        .expect("Database connection failed")
}
