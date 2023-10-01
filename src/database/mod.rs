use crate::config::DbConfig;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection(config: DbConfig) -> PgConnection {
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        config.username, config.password, config.host, config.database
    );
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
