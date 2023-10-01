use std::env;
use dotenv::dotenv;

pub struct DbConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        let host = env::var("DB_HOST").expect("Missing DB_HOST variable");
        let port = env::var("DB_PORT").expect("Missing DB_PORT variable");
        let username = env::var("DB_USERNAME").expect("Missing DB_USERNAME variable");
        let password = env::var("DB_PASSWORD").expect("Missing DB_PASSWORD variable");
        let database = env::var("DB_DATABASE").expect("Missing DB_DATABASE variable");

        Ok(DbConfig {
            host,
            port,
            username,
            password,
            database,
        })
    }
}
