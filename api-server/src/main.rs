mod application_config;
mod controllers;
mod domain;
mod repositories;
mod server;
mod use_cases;

use crate::application_config::ApplicationConfig;
use server::Server;

fn main() {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("error: unable to load .env file");
        }
    }

    Server::new(ApplicationConfig::new()).start();
}
