use crate::domain::task::task::Task;
use crate::domain::task::task_description::TaskDescription;
use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_repository::{TaskRepository, TaskRepositoryError};
use crate::domain::task::task_status::TaskStatus;
use crate::domain::task::task_title::TaskTitle;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct TaskInMemoryRepository {
    data: RefCell<HashMap<TaskId, Task>>,
}

impl TaskInMemoryRepository {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }
}

impl TaskRepository for TaskInMemoryRepository {
    fn list(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        Ok(self.data.borrow().values().cloned().collect())
    }

    fn get_by_id(&self, id: &TaskId) -> Result<Task, TaskRepositoryError> {
        self.data
            .borrow()
            .get(id)
            .cloned()
            .ok_or(TaskRepositoryError::NotFound)
    }

    fn register(&self, task: Task) -> Result<Task, TaskRepositoryError> {
        let mut data = self.data.borrow_mut();

        if data.contains_key(&task.id) {
            return Err(TaskRepositoryError::AlreadyExists);
        }

        data.insert(task.id.clone(), task.clone());
        Ok(task)
    }

    fn update(&self, task: Task) -> Result<Task, TaskRepositoryError> {
        let mut data = self.data.borrow_mut();

        if !data.contains_key(&task.id) {
            return Err(TaskRepositoryError::NotFound);
        }

        data.insert(task.id.clone(), task.clone());
        Ok(task)
    }

    fn delete(&self, task_id: &TaskId) -> Result<(), TaskRepositoryError> {
        let mut data = self.data.borrow_mut();

        data.remove(task_id).ok_or(TaskRepositoryError::NotFound)?;
        Ok(())
    }
}

#[cfg(test)]
impl TaskInMemoryRepository {
    pub fn register_test_data(&mut self) -> Vec<TaskId> {
        let mut ids = Vec::new();
        let data = [("AAA", "AAA"), ("BBB", "BBB"), ("CCC", "CCC")];

        let mut map = self.data.borrow_mut();

        for (title, description) in data {
            let task = Task::new(
                TaskId::new(),
                TaskTitle::try_from(title).unwrap(),
                TaskDescription::try_from(description).unwrap(),
                TaskStatus::Todo,
            );

            ids.push(task.id.clone());
            map.insert(task.id.clone(), task);
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
        assert_eq!(list.unwrap().len(), 3);
    }

    #[test]
    fn list_when_tasks_are_not_registered_then_returns_empty_list() {
        let repository = TaskInMemoryRepository::new();
        let list = repository.list();
        assert_eq!(list.unwrap().len(), 0);
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
        let _ = repository.delete(&task_id);

        let list = repository.list();
        assert_eq!(list.unwrap().len(), 2);
    }
}
