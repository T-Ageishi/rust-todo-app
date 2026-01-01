use crate::domain::task::task::Task;
use crate::domain::task::task_description::TaskDescription;
use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_repository::{TaskRepository, TaskRepositoryError};
use crate::domain::task::task_status::TaskStatus;
use crate::domain::task::task_title::TaskTitle;
use crate::repositories::app_db::AppDb;
use mysql::params;
use mysql::prelude::*;

pub struct TaskMysqlRepository<'a> {
    db: &'a AppDb,
}

impl<'a> TaskMysqlRepository<'a> {
    pub fn new(db: &'a AppDb) -> Self {
        Self { db }
    }
}

impl<'a> TaskRepository for TaskMysqlRepository<'a> {
    fn list(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        let mut conn = self.db.get_conn();

        let tasks = conn
            .query_map(
                "SELECT id, title, description, status FROM tasks",
                |(id, title, description, status): (String, String, String, i32)| {
                    Task::new(
                        TaskId::try_from(id.as_str()).unwrap(),
                        TaskTitle::try_from(title.as_str()).unwrap(),
                        TaskDescription::try_from(description.as_str()).unwrap(),
                        TaskStatus::try_from(status).unwrap(),
                    )
                },
            )
            .map_err(|e| TaskRepositoryError::DbError(e.to_string()))?;

        Ok(tasks)
    }

    fn get_by_id(&self, id: &TaskId) -> Result<Task, TaskRepositoryError> {
        let mut conn = self.db.get_conn();

        let row: Option<(String, String, String, i32)> = conn
            .exec_first(
                "SELECT id, title, description, status FROM tasks WHERE id = :id",
                params! {
                    "id" => id.to_string(),
                },
            )
            .map_err(|e| TaskRepositoryError::DbError(e.to_string()))?;

        let (id, title, description, status) = row.ok_or(TaskRepositoryError::NotFound)?;

        Ok(Task::new(
            TaskId::try_from(id.as_str()).unwrap(),
            TaskTitle::try_from(title.as_str()).unwrap(),
            TaskDescription::try_from(description.as_str()).unwrap(),
            TaskStatus::try_from(status).unwrap(),
        ))
    }

    fn register(&self, task: Task) -> Result<Task, TaskRepositoryError> {
        let mut conn = self.db.get_conn();

        conn.exec_drop(
            "INSERT INTO tasks (id, title, description, status)
             VALUES (:id, :title, :description, :status)",
            params! {
                "id" => task.id.to_string(),
                "title" => task.title.to_string(),
                "description" => task.description.to_string(),
                "status" => task.status.to_int(),
            },
        )
        .map_err(|e| TaskRepositoryError::DbError(e.to_string()))?;

        Ok(task)
    }

    fn update(&self, task: Task) -> Result<Task, TaskRepositoryError> {
        let mut conn = self.db.get_conn();

        conn.exec_drop(
            "UPDATE tasks
             SET title = :title, description = :description, status = :status
             WHERE id = :id",
            params! {
                "id" => task.id.to_string(),
                "title" => task.title.to_string(),
                "description" => task.description.to_string(),
                "status" => task.status.to_int(),
            },
        )
        .map_err(|e| TaskRepositoryError::DbError(e.to_string()))?;

        Ok(task)
    }

    fn delete(&self, task_id: &TaskId) -> Result<(), TaskRepositoryError> {
        let mut conn = self.db.get_conn();

        conn.exec_drop(
            "DELETE FROM tasks WHERE id = :id",
            params! {
                "id" => task_id.to_string(),
            },
        )
        .map_err(|e| TaskRepositoryError::DbError(e.to_string()))?;

        Ok(())
    }
}
