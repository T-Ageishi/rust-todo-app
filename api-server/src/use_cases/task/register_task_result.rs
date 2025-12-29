use crate::domain::task::task::Task;

#[derive(Debug)]
pub struct RegisterTaskResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: i32,
}

impl RegisterTaskResult {
    pub fn from(task: &Task) -> Self {
        Self {
            id: task.id.to_string(),
            title: task.title.to_string(),
            description: task.description.to_string(),
            status: task.status.to_int(),
        }
    }
}
