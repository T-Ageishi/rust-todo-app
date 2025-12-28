use crate::domain::task::task::Task;
use crate::domain::task::task_description::TaskDescription;
use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_repository::TaskRepository;
use crate::domain::task::task_status::TaskStatus;
use crate::domain::task::task_title::TaskTitle;
use crate::use_cases::task::update_task_command::UpdateTaskCommand;
use crate::use_cases::task::update_task_result::UpdateTaskResult;
#[cfg(test)]
use crate::repositories::task::task_in_memory_repository::TaskInMemoryRepository;
#[cfg(test)]
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
pub enum UpdateTaskError {
    InvalidID,
    InvalidTitle,
    InvalidDescription,
    InvalidStatus,
    TaskNotFound,
}

#[derive(Debug)]
pub struct UpdateTask<'a, T: TaskRepository> {
    repository: &'a mut T,
}

impl<'a, T: TaskRepository> UpdateTask<'a, T> {
    pub fn execute(
        &mut self,
        command: UpdateTaskCommand,
    ) -> Result<UpdateTaskResult, UpdateTaskError> {
        let id = match TaskId::try_from(command.id.to_string().as_str()) {
            Ok(id) => id,
            Err(_) => return Err(UpdateTaskError::InvalidID),
        };

        let task = match self.repository.get_by_id(&id) {
            Ok(task) => task,
            Err(_) => return Err(UpdateTaskError::TaskNotFound),
        };

        let mut task = Task::new(
            task.id.clone(),
            TaskTitle::try_from(task.title.to_string().as_str()).unwrap(),
            TaskDescription::try_from(task.description.to_string().as_str()).unwrap(),
            TaskStatus::try_from(task.status.to_int()).unwrap(),
        );
        if command.title.is_some() {
            let title = match TaskTitle::try_from(command.title.unwrap().as_str()) {
                Ok(title) => title,
                Err(_) => return Err(UpdateTaskError::InvalidTitle),
            };
            task = task.change_title(title);
        }
        if command.description.is_some() {
            let description = match TaskDescription::try_from(command.description.unwrap().as_str())
            {
                Ok(description) => description,
                Err(_) => return Err(UpdateTaskError::InvalidDescription),
            };
            task = task.change_description(description);
        }
        if command.status.is_some() {
            let status = match TaskStatus::try_from(command.status.unwrap()) {
                Ok(status) => status,
                Err(_) => return Err(UpdateTaskError::InvalidStatus),
            };
            task = task.change_status(status);
        }

        match self.repository.update(task) {
            Ok(task) => Ok(UpdateTaskResult::from(task)),
            Err(_) => Err(UpdateTaskError::TaskNotFound),
        }
    }
}

#[test]
fn execute_when_valid_input_then_returns_result() {
    let mut repository = TaskInMemoryRepository::new();
    let ids = repository.register_test_data();

    let mut update_task = UpdateTask {
        repository: &mut repository,
    };
    let command = UpdateTaskCommand::new(
        ids.get(0).unwrap().to_string().as_str(),
        Some(String::from("New Task Title").as_str()),
        Some(String::from("New Task Description").as_str()),
        Some(2),
    ).unwrap();
    let result = update_task.execute(command).ok().unwrap();
    assert_eq!(result.id, ids.get(0).unwrap().to_string());
    assert_eq!(result.title, String::from("New Task Title"));
    assert_eq!(result.description, String::from("New Task Description"));
    assert_eq!(result.status, 2);
}

#[test]
fn execute_when_invalid_task_title_then_returns_error() {
    let mut repository = TaskInMemoryRepository::new();
    let ids = repository.register_test_data();

    let mut update_task = UpdateTask {
        repository: &mut repository,
    };
    let command = UpdateTaskCommand::new(
        ids.get(0).unwrap().to_string().as_str(),
        Some(String::from("").as_str()),
        Some(String::from("New Task Description").as_str()),
        Some(2),
    ).unwrap();
    let result = update_task.execute(command).err().unwrap();
    assert_eq!(result, UpdateTaskError::InvalidTitle);
}

#[test]
fn execute_when_invalid_task_description_then_returns_error() {
    let mut repository = TaskInMemoryRepository::new();
    let ids = repository.register_test_data();

    let mut update_task = UpdateTask {
        repository: &mut repository,
    };
    let command = UpdateTaskCommand::new(
        ids.get(0).unwrap().to_string().as_str(),
        Some(String::from("New Task Title").as_str()),
        Some(String::from("").as_str()),
        Some(2),
    ).unwrap();
    let result = update_task.execute(command).err().unwrap();
    assert_eq!(result, UpdateTaskError::InvalidDescription);
}

#[test]
fn execute_when_invalid_task_status_then_returns_error() {
    let mut repository = TaskInMemoryRepository::new();
    let ids = repository.register_test_data();

    let mut update_task = UpdateTask {
        repository: &mut repository,
    };
    let command = UpdateTaskCommand::new(
        ids.get(0).unwrap().to_string().as_str(),
        Some(String::from("New Task Title").as_str()),
        Some(String::from("New Task Description").as_str()),
        Some(4),
    ).unwrap();
    let result = update_task.execute(command).err().unwrap();
    assert_eq!(result, UpdateTaskError::InvalidStatus);
}

#[test]
fn execute_when_task_not_found_then_returns_error() {
    let mut repository = TaskInMemoryRepository::new();
    let mut update_task = UpdateTask {
        repository: &mut repository,
    };
    let command = UpdateTaskCommand::new(
        Uuid::new_v4().to_string().as_str(),
        Some(String::from("New Task Title").as_str()),
        Some(String::from("New Task Description").as_str()),
        Some(4),
    ).unwrap();
    let result = update_task.execute(command).err().unwrap();
    assert_eq!(result, UpdateTaskError::TaskNotFound);
}
