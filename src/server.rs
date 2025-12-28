use crate::controllers::task_controller::TaskController;
use crate::repositories::task::task_in_memory_repository::TaskInMemoryRepository;
use tiny_http::{Method, Response};

pub struct Server {
    //
}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {
        let server = tiny_http::Server::http("127.0.0.1:8080").unwrap();

        let mut repository = TaskInMemoryRepository::new();

        loop {
            let mut request = match server.recv() {
                Ok(rq) => rq,
                Err(e) => {
                    println!("error: {}", e);
                    break;
                }
            };

            match (request.url(), request.method()) {
                ("/api/v1/tasks", Method::Get) => {
                    let controller = TaskController::new(&mut repository);
                    let response = controller.get();
                    match request.respond(response) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("error (GET /api/v1/tasks): {}", e);
                        }
                    }
                }
                ("/api/v1/tasks", Method::Post) => {
                    let mut controller = TaskController::new(&mut repository);
                    let response = controller.post(&mut request);
                    match request.respond(response) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("error(POST /api/v1/tasks): {}", e);
                        }
                    }
                }
                ("/api/v1/tasks", Method::Patch) => {
                    let mut controller = TaskController::new(&mut repository);
                    let response = controller.patch(&mut request);
                    match request.respond(response) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("error (Patch /api/v1/tasks): {}", e);
                        }
                    }
                }
                _ => {
                    let _ = request.respond(Response::new_empty(tiny_http::StatusCode(404)));
                }
            }
        }
    }
}
