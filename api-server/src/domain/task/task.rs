use crate::domain::task::task_description::TaskDescription;
use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_status::TaskStatus;
use crate::domain::task::task_title::TaskTitle;

#[derive(Clone)]
pub struct Task {
    pub id: TaskId,
    pub title: TaskTitle,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(
        id: TaskId,
        title: TaskTitle,
        description: TaskDescription,
        status: TaskStatus,
    ) -> Task {
        Task {
            id,
            title,
            description,
            status,
        }
    }

    pub fn change_title(self, title: TaskTitle) -> Self {
        Self { title, ..self }
    }

    pub fn change_description(self, description: TaskDescription) -> Self {
        Self {
            description,
            ..self
        }
    }

    pub fn change_status(self, status: TaskStatus) -> Self {
        Self { status, ..self }
    }
}
