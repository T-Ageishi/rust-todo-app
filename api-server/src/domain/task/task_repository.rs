use crate::domain::task::task::Task;
use crate::domain::task::task_id::TaskId;

pub trait TaskRepository {
    fn list(&self) -> Result<Vec<Task>, TaskRepositoryError>;
    fn get_by_id(&self, id: &TaskId) -> Result<Task, TaskRepositoryError>;
    fn register(&self, task: Task) -> Result<Task, TaskRepositoryError>;
    fn update(&self, task: Task) -> Result<Task, TaskRepositoryError>;
    fn delete(&self, task_id: &TaskId) -> Result<(), TaskRepositoryError>;
}

#[derive(Debug)]
pub enum TaskRepositoryError {
    AlreadyExists,
    NotFound,
    DbError(String),
}
