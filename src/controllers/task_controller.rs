use crate::domain::task::task::Task;
use crate::domain::task::task_repository::TaskRepository;
use crate::use_cases::task::register_task::{RegisterTask, RegisterTaskError};
use crate::use_cases::task::register_task_command::RegisterTaskCommand;
use crate::use_cases::task::update_task::{UpdateTask, UpdateTaskError};
use crate::use_cases::task::update_task_command::UpdateTaskCommand;
use serde::{Deserialize, Serialize};
use std::io::Read;
use tiny_http::{Request, Response, StatusCode};

#[derive(Debug, Serialize, Deserialize)]
struct TaskDTO {
    id: String,
    title: String,
    description: String,
    status: i32,
}

#[derive(Serialize, Deserialize)]
struct TaskPostInput {
    title: String,
    description: String,
    status: i32,
}
#[derive(Serialize, Deserialize)]
struct TaskPostOutput {
    data: TaskDTO,
}

#[derive(Serialize, Deserialize)]
struct TaskListOutput {
    data: Vec<TaskDTO>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskPatchInput {
    id: String,
    title: Option<String>,
    description: Option<String>,
    status: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize)]
struct TaskPatchOutput {
    data: TaskDTO,
}

impl TaskListOutput {
    fn from(task_list: &Vec<&Task>) -> Self {
        let mut list = Vec::new();
        for task in task_list {
            list.push(TaskDTO {
                id: task.id.to_string(),
                title: task.title.to_string(),
                description: task.description.to_string(),
                status: task.status.to_int(),
            })
        }

        Self { data: list }
    }
}

#[derive(Debug)]
pub struct TaskController<'a, T: TaskRepository> {
    repository: &'a mut T,
}

impl<'a, T: TaskRepository> TaskController<'a, T> {
    pub fn new(repository: &'a mut T) -> Self {
        Self { repository }
    }

    pub fn get(&self) -> Response<std::io::Cursor<Vec<u8>>> {
        let tasks = self.repository.list();
        let task_list_output = TaskListOutput::from(&tasks);
        let json = serde_json::to_string(&task_list_output).unwrap();

        Response::from_string(String::from(json)).with_status_code(200)
    }

    pub fn post(&mut self, request: &mut Request) -> Response<std::io::Cursor<Vec<u8>>> {
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
        let mut use_case = RegisterTask::new(self.repository);
        let result = match use_case.execute(command) {
            Ok(result) => result,
            Err(ref e) => {
                return match e {
                    RegisterTaskError::InvalidTitle => {
                        Response::from_string(String::from("Invalid task title input"))
                            .with_status_code(StatusCode::from(400))
                    }
                    RegisterTaskError::InvalidDescription => {
                        Response::from_string(String::from("Invalid task description input"))
                            .with_status_code(StatusCode::from(400))
                    }
                    RegisterTaskError::RepositoryError => {
                        Response::from_string(String::from("Error occurred during saving task"))
                            .with_status_code(StatusCode::from(500))
                    }
                };
            }
        };

        let payload = TaskPostOutput {
            data: TaskDTO {
                id: result.id,
                title: result.title,
                description: result.description,
                status: result.status,
            },
        };
        Response::from_string(serde_json::to_string(&payload).unwrap()).with_status_code(200)
    }

    pub fn patch(&mut self, request: &mut Request) -> Response<std::io::Cursor<Vec<u8>>> {
        let mut body = String::new();
        request.as_reader().read_to_string(&mut body).unwrap();

        let payload: TaskPatchInput = match serde_json::from_str(body.as_str()) {
            Ok(payload) => payload,
            Err(_) => {
                return Response::from_string(String::from("Invalid request body"))
                    .with_status_code(StatusCode::from(400));
            }
        };

        let command = match UpdateTaskCommand::new(
            payload.id.as_str(),
            payload.title.as_deref(),
            payload.description.as_deref(),
            payload.status,
        ) {
            Ok(command) => command,
            Err(_) => return Response::from_string(String::from("Invalid request payload")),
        };
        let mut use_case = UpdateTask::new(self.repository);
        let result = match use_case.execute(command) {
            Ok(result) => result,
            Err(ref e) => {
                return match e {
                    UpdateTaskError::InvalidID => {
                        Response::from_string(String::from("Invalid task ID input"))
                    }
                    UpdateTaskError::InvalidTitle => {
                        return Response::from_string(String::from("Invalid task title input"));
                    }
                    UpdateTaskError::InvalidDescription => {
                        Response::from_string(String::from("Invalid task description input"))
                    }
                    UpdateTaskError::InvalidStatus => {
                        Response::from_string(String::from("Invalid task status input"))
                    }
                    UpdateTaskError::TaskNotFound => {
                        Response::from_string(String::from("Task not found"))
                    }
                };
            }
        };

        let payload = TaskPatchOutput {
            data: TaskDTO {
                id: result.id,
                title: result.title,
                description: result.description,
                status: result.status,
            },
        };
        Response::from_string(serde_json::to_string(&payload).unwrap()).with_status_code(200)
    }
}
