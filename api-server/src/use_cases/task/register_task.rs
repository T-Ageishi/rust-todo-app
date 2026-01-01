use crate::domain::task::task::Task;
use crate::domain::task::task_description::TaskDescription;
use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_repository::TaskRepository;
use crate::domain::task::task_status::TaskStatus;
use crate::domain::task::task_title::TaskTitle;
use crate::use_cases::task::register_task_command::RegisterTaskCommand;
use crate::use_cases::task::register_task_result::RegisterTaskResult;

#[derive(Debug, Eq, PartialEq)]
pub enum RegisterTaskError {
    InvalidTitle,
    InvalidDescription,
    InvalidStatus,
    RepositoryError,
}

#[derive(Debug)]
pub struct RegisterTask<'a, T: TaskRepository> {
    repository: &'a mut T,
}

impl<'a, T: TaskRepository> RegisterTask<'a, T> {
    pub fn new(repository: &'a mut T) -> Self {
        Self { repository }
    }

    pub fn execute(
        &mut self,
        command: RegisterTaskCommand,
    ) -> Result<RegisterTaskResult, RegisterTaskError> {
        let id = TaskId::new();
        let title = match TaskTitle::try_from(command.title()) {
            Ok(title) => title,
            Err(_) => return Err(RegisterTaskError::InvalidTitle),
        };
        let description = match TaskDescription::try_from(command.description()) {
            Ok(description) => description,
            Err(_) => return Err(RegisterTaskError::InvalidDescription),
        };
        let status = match TaskStatus::try_from(command.status()) {
            Ok(status) => status,
            Err(_) => return Err(RegisterTaskError::InvalidStatus),
        };

        let task = Task::new(id, title, description, status);
        let task = match self.repository.register(task) {
            Ok(task) => task,
            Err(_) => return Err(RegisterTaskError::RepositoryError),
        };

        Ok(RegisterTaskResult::from(&task))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::task::task_in_memory_repository::TaskInMemoryRepository;

    #[test]
    fn execute_when_valid_input_then_returns_registered_task() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let command = RegisterTaskCommand::new("Task Title", "Task Description", 2);
        let result = register_task.execute(command).ok().unwrap();
        assert_eq!(result.title, "Task Title");
        assert_eq!(result.description, "Task Description");
        assert_eq!(result.status, 2);
    }

    #[test]
    fn execute_when_task_title_is_empty_then_returns_error() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let command = RegisterTaskCommand::new("", "Task Description", 2);
        let result = register_task.execute(command).err().unwrap();
        assert_eq!(result, RegisterTaskError::InvalidTitle);
    }

    #[test]
    fn execute_when_task_title_is_too_long_then_returns_error() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let task_title = String::from("A").repeat(65);
        let command = RegisterTaskCommand::new(task_title.as_str(), "Task Description", 2);
        let result = register_task.execute(command).err().unwrap();
        assert_eq!(result, RegisterTaskError::InvalidTitle);
    }

    #[test]
    fn execute_when_task_description_is_empty_then_returns_error() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let command = RegisterTaskCommand::new("Task Title", "", 2);
        let result = register_task.execute(command).err().unwrap();
        assert_eq!(result, RegisterTaskError::InvalidDescription);
    }

    #[test]
    fn execute_when_task_description_is_too_long_then_returns_error() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let task_description = String::from("A").repeat(257);
        let command = RegisterTaskCommand::new("Task Title", task_description.as_str(), 2);
        let result = register_task.execute(command).err().unwrap();
        assert_eq!(result, RegisterTaskError::InvalidDescription);
    }

    #[test]
    fn execute_when_task_status_is_invalid_then_returns_error() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let command = RegisterTaskCommand::new("Task Title", "Task Description", 5);
        let result = register_task.execute(command).err().unwrap();
        assert_eq!(result, RegisterTaskError::InvalidStatus);
    }

    #[test]
    fn execute_when_task_status_is_empty_then_returns_error() {
        let mut repository = TaskInMemoryRepository::new();
        let mut register_task = RegisterTask::new(&mut repository);
        let command = RegisterTaskCommand::new("Task Title", "Task Description", 5);
        let result = register_task.execute(command).err().unwrap();
        assert_eq!(result, RegisterTaskError::InvalidStatus);
    }
}
