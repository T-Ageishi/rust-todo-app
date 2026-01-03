use crate::application_config::ApplicationConfig;
use crate::controllers::task_controller::TaskController;
use crate::repositories::app_db::AppDb;
use crate::repositories::task::task_mysql_repository::TaskMysqlRepository;
use tiny_http::{Method, Response};

pub struct Server {
    config: ApplicationConfig,
    app_db: AppDb,
}

impl Server {
    pub fn new() -> Self {
        let config = ApplicationConfig::new();
        let app_db = match AppDb::new(config.db_config()) {
            Ok(app_db) => app_db,
            Err(e) => panic!("{}", e),
        };

        Self { config, app_db }
    }

    pub fn start(&self) {
        let server = tiny_http::Server::http(format!(
            "{}:{}",
            self.config.server_config().addr(),
            self.config.server_config().port()
        ))
        .unwrap();

        println!(
            "Listening for requests at {}://{}",
            self.config.server_config().scheme(),
            server.server_addr()
        );

        let mut repository = TaskMysqlRepository::new(&self.app_db);

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
                ("/api/v1/tasks", Method::Delete) => {
                    let mut controller = TaskController::new(&mut repository);
                    let response = controller.delete(&mut request);
                    match request.respond(response) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("error (Delete /api/v1/tasks): {}", e);
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
