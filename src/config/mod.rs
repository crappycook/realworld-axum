use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db: DatabaseConfig,
}

impl AppConfig {
    pub fn load_from_file(path: String) -> Self {
        let mut file = File::open(path).expect("Failed to open config file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to read config file");
        let app_config: AppConfig = toml::from_str(&buf).expect("Failed to parse config file");
        app_config
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub max_connections: u32,
    pub database: String,
}

impl DatabaseConfig {
    pub fn get_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::database;

    #[tokio::test]
    async fn test_db_conn() {
        use super::*;

        let app_config = AppConfig::load_from_file("src/config/config.toml".to_string());
        println!("{:?}", app_config);
        let conn = database::init_conn_from_config(&app_config.db).await;
        conn.ping().await.expect("Database ping failed.");
    }
}
