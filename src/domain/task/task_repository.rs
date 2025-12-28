use crate::domain::task::task::Task;
use crate::domain::task::task_id::TaskId;

pub trait TaskRepository {
    fn list(&self) -> Vec<&Task>;
    fn get_by_id(&self, id: &TaskId) -> Result<&Task, TaskRepositoryError>;
    fn register(&mut self, task: Task) -> Result<&Task, TaskRepositoryError>;
    fn update(&mut self, task: Task) -> Result<&Task, TaskRepositoryError>;
    fn delete(&mut self, task_id: &TaskId);
}

pub enum TaskRepositoryError {
    NotFound,
}
