use serde::Deserialize;

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
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
