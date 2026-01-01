use mysql::{Pool, PooledConn};
use std::time::Duration;
use std::{env, thread};

pub struct AppDb {
    pool: Pool,
}

impl AppDb {
    pub fn init() -> Self {
        let user = env::var("MYSQL_USER").expect("MYSQL_USER not set");
        let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD not set");
        let host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let url = format!("mysql://{}:{}@{}:3306/app_db", user, password, host);

        let mut last_err = None;
        for attempt in 1..10 {
            match Pool::new(url.as_str()) {
                Ok(pool) => match pool.get_conn() {
                    Ok(_) => {
                        println!("MySQL connected (attempt {})", attempt);
                        return Self { pool };
                    }
                    Err(e) => {
                        last_err = Some(e.to_string());
                    }
                },
                Err(e) => {
                    last_err = Some(e.to_string());
                }
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
            "failed to connect to mysql after 10 attempts: {:?}",
            last_err
        );
    }

    pub fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }
}
