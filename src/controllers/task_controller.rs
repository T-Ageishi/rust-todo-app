use crate::domain::task::task_repository::TaskRepository;
use crate::use_cases::task::register_task::{RegisterTask, ResisterTaskError};
use crate::use_cases::task::register_task_command::RegisterTaskCommand;
use http::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TaskPostInput {
    title: String,
    description: String,
    status: i32,
}
#[derive(Serialize, Deserialize)]
struct TaskPostOutput {
    id: String,
    title: String,
    description: String,
    status: i32,
}

#[derive(Debug)]
pub struct TaskController<T: TaskRepository> {
    repository: T,
}

impl<T: TaskRepository> TaskController<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn post(&mut self, request: Request<String>) -> Response<String> {
        let payload: TaskPostInput = match serde_json::from_str(request.body()) {
            Ok(payload) => payload,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(String::from("Invalid request body"))
                    .unwrap();
            }
        };

        let command =
            RegisterTaskCommand::new(payload.title.as_str(), payload.description.as_str());
        let mut use_case = RegisterTask::new(&mut self.repository);
        let result = match use_case.execute(command) {
            Ok(result) => result,
            Err(ref e) => {
                return match e {
                    ResisterTaskError::InvalidTitle => Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(String::from("Invalid task title input"))
                        .unwrap(),
                    ResisterTaskError::InvalidDescription => Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(String::from("Invalid task description input"))
                        .unwrap(),
                    ResisterTaskError::RepositoryError => Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(String::from("Error occurred during saving task"))
                        .unwrap(),
                };
            }
        };

        let payload = TaskPostOutput {
            id: result.id,
            title: result.title,
            description: result.description,
            status: result.status,
        };
        Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&payload).unwrap())
            .unwrap()
    }
}
