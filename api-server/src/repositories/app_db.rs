use crate::application_config::DbConfig;
use mysql::{Pool, PooledConn};
use std::thread;
use std::time::Duration;

pub struct AppDb {
    pool: Pool,
}

impl AppDb {
    pub fn new(db_config: &DbConfig) -> Result<Self, mysql::Error> {
        let url = db_config.database_url("app_db");

        let mut last_err = None;
        for attempt in 1..10 {
            match Pool::new(url.as_str()) {
                Ok(pool) => {
                    println!("info: connected to DB (attempt {})", attempt);
                    return Ok(AppDb { pool });
                }
                Err(e) => last_err = Some(e.to_string()),
            }

            let backoff_sec = 1 << attempt;
            eprintln!(
                "waiting for mysql... attempt {}/10, retry in {}s ({})",
                attempt,
                backoff_sec,
                last_err.as_deref().unwrap_or("unknown error"),
            );

            thread::sleep(Duration::from_secs(backoff_sec));
        }

        panic!(
            "error: failed to connect to mysql after 10 attempts: {:?}",
            last_err
        );
    }

    pub fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }
}
