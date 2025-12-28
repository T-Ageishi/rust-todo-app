use crate::domain::task::task_repository::TaskRepository;
use crate::use_cases::task::register_task::{RegisterTask, ResisterTaskError};
use crate::use_cases::task::register_task_command::RegisterTaskCommand;
use serde::{Deserialize, Serialize};
use tiny_http::{Request, Response, StatusCode};

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

    pub fn post(
        &mut self,
        request: &mut Request,
    ) -> Response<std::io::Cursor<Vec<u8>>> {
        let mut body = String::new();
        request.as_reader().read_to_string(&mut body).unwrap();

        let payload: TaskPostInput = match serde_json::from_str(body.as_str()) {
            Ok(payload) => payload,
            Err(_) => {
                return Response::from_string(String::from("Invalid request body"))
                    .with_status_code(StatusCode::from(400));
            }
        };

        let command =
            RegisterTaskCommand::new(payload.title.as_str(), payload.description.as_str());
        let mut use_case = RegisterTask::new(&mut self.repository);
        let result = match use_case.execute(command) {
            Ok(result) => result,
            Err(ref e) => {
                return match e {
                    ResisterTaskError::InvalidTitle => {
                        Response::from_string(String::from("Invalid task title input"))
                            .with_status_code(StatusCode::from(400))
                    }
                    ResisterTaskError::InvalidDescription => {
                        Response::from_string(String::from("Invalid task description input"))
                            .with_status_code(StatusCode::from(400))
                    }
                    ResisterTaskError::RepositoryError => {
                        Response::from_string(String::from("Error occurred during saving task"))
                            .with_status_code(StatusCode::from(500))
                    }
                };
            }
        };

        let payload = TaskPostOutput {
            id: result.id,
            title: result.title,
            description: result.description,
            status: result.status,
        };
        Response::from_string(serde_json::to_string(&payload).unwrap()).with_status_code(200)
    }
}
