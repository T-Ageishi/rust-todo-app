use crate::domain::task::task::Task;
#[cfg(test)]
use crate::domain::task::task_description::TaskDescription;
use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_repository::{TaskRepository, TaskRepositoryError};
#[cfg(test)]
use crate::domain::task::task_status::TaskStatus;
#[cfg(test)]
use crate::domain::task::task_title::TaskTitle;
use std::collections::HashMap;

pub struct TaskInMemoryRepository {
    data: HashMap<TaskId, Task>,
}

impl TaskInMemoryRepository {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl TaskRepository for TaskInMemoryRepository {
    fn list(&self) -> Vec<&Task> {
        self.data.values().collect()
    }

    fn get_by_id(&self, id: &TaskId) -> Result<&Task, TaskRepositoryError> {
        match self.data.get(&id) {
            Some(task) => Ok(task),
            None => Err(TaskRepositoryError::NotFound),
        }
    }

    fn register(&mut self, task: Task) -> Result<&Task, TaskRepositoryError> {
        let id = task.id.clone();
        if !self.data.contains_key(&task.id) {
            self.data.insert(task.id.clone(), task);
        }

        let task = match self.get_by_id(&id) {
            Ok(task) => task,
            Err(_) => return Err(TaskRepositoryError::NotFound),
        };

        Ok(task)
    }

    fn update(&mut self, task: Task) -> Result<&Task, TaskRepositoryError> {
        let id = task.id.clone();
        if self.data.contains_key(&task.id) {
            self.data.insert(task.id.clone(), task);
        }

        let task = match self.get_by_id(&id) {
            Ok(task) => task,
            Err(_) => return Err(TaskRepositoryError::NotFound),
        };

        Ok(task)
    }

    fn delete(&mut self, task_id: &TaskId) {
        if self.data.contains_key(task_id) {
            self.data.remove(task_id);
        }
    }
}

#[cfg(test)]
impl TaskInMemoryRepository {
    pub fn register_test_data(&mut self) -> Vec<TaskId> {
        let mut ids = Vec::new();
        let data = [("AAA", "AAA"), ("BBB", "BBB"), ("CCC", "CCC")];
        for d in data.iter() {
            let task = Task::new(
                TaskId::new(),
                TaskTitle::try_from(d.0).unwrap(),
                TaskDescription::try_from(d.1).unwrap(),
                TaskStatus::Todo,
            );
            ids.push(task.id.clone());
            self.data.insert(task.id.clone(), task);
        }
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_when_tasks_are_registered_then_returns_task_list() {
        let mut repository = TaskInMemoryRepository::new();
        repository.register_test_data();

        let list = repository.list();
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn list_when_tasks_are_not_registered_then_returns_empty_list() {
        let repository = TaskInMemoryRepository::new();
        let list = repository.list();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn get_by_id_when_valid_value_then_returns_task() {
        let mut repository = TaskInMemoryRepository::new();
        let ids = repository.register_test_data();
        let task = repository.get_by_id(&ids[0]).ok().unwrap();
        assert_eq!(task.id, ids[0]);
        assert_eq!(task.title.to_string(), "AAA");
        assert_eq!(task.description.to_string(), "AAA");
    }

    #[test]
    fn register_when_valid_value_then_returns_task() {
        let mut repository = TaskInMemoryRepository::new();
        repository.register_test_data();

        let task_id = TaskId::new();
        let task_title = TaskTitle::try_from("DDD").unwrap();
        let task_description = TaskDescription::try_from("DDD").unwrap();
        let task = Task::new(task_id, task_title, task_description, TaskStatus::Todo);
        let task = repository.register(task).ok().unwrap();
        assert_eq!(task.title.to_string(), "DDD");
        assert_eq!(task.description.to_string(), "DDD");
    }

    #[test]
    fn update_when_valid_value_then_returns_task() {
        let mut repository = TaskInMemoryRepository::new();
        let ids = repository.register_test_data();

        let task_id = ids[0].clone();
        let task_title = TaskTitle::try_from("AAA2").unwrap();
        let task_description = TaskDescription::try_from("AAA2").unwrap();
        let task = Task::new(task_id, task_title, task_description, TaskStatus::Todo);
        let task = repository.update(task).ok().unwrap();
        assert_eq!(task.title.to_string(), "AAA2");
        assert_eq!(task.description.to_string(), "AAA2");
    }

    #[test]
    fn delete_when_valid_id_then_returns_empty_task() {
        let mut repository = TaskInMemoryRepository::new();
        let ids = repository.register_test_data();

        let task_id = ids[0].clone();
        repository.delete(&task_id);

        let list = repository.list();
        assert_eq!(list.len(), 2);
    }
}
