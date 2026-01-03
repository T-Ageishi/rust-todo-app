mod application_config;
mod controllers;
mod domain;
mod repositories;
mod server;
mod use_cases;

use server::Server;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("error: unable to load .env file");
        }
    }

    Server::new().start();
}
