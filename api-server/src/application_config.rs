use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ApplicationConfig {
    server: ServerConfig,
    db: DbConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    scheme: String,
    addr: String,
    port: String,
}

#[derive(Debug)]
pub struct DbConfig {
    host: String,
    port: String,
    user: String,
    password: String,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        let path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("server_config.json");
        let server_config =
            serde_json::from_reader(std::fs::File::open(path_buf.as_path()).unwrap()).unwrap();

        let db_config = DbConfig {
            host: dotenvy::var("DB_HOST").unwrap(),
            port: dotenvy::var("DB_PORT").unwrap(),
            user: dotenvy::var("DB_USER").unwrap(),
            password: dotenvy::var("DB_USER_PASSWORD").unwrap(),
        };

        Self {
            server: server_config,
            db: db_config,
        }
    }

    pub fn server_config(&self) -> &ServerConfig {
        &self.server
    }

    pub fn db_config(&self) -> &DbConfig {
        &self.db
    }
}

impl ServerConfig {
    pub fn addr(&self) -> &str {
        &self.addr
    }
    pub fn port(&self) -> &str {
        &self.port
    }
    pub fn scheme(&self) -> &str {
        &self.scheme
    }
}

impl DbConfig {
    pub fn database_url(&self, database: &str) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, database
        )
    }
}
