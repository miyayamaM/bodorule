#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn to_database_url(&self) -> String {
        format!(
            "{}://{}:{}@{}/{}",
            self.protocol, self.username, self.password, self.host, self.database
        )
    }
}
