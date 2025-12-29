mod domain;
mod repositories;
mod use_cases;
mod controllers;
mod server;

use server::Server;

fn main() {
    Server::new().start();
}
