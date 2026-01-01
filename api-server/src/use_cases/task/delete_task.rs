use crate::domain::task::task_id::TaskId;
use crate::domain::task::task_repository::TaskRepository;
use crate::use_cases::task::delete_task_command::DeleteTaskCommand;

pub struct DeleteTask<'a, T: TaskRepository> {
    repository: &'a mut T,
}

impl<'a, T: TaskRepository> DeleteTask<'a, T> {
    pub fn new(repository: &'a mut T) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, command: DeleteTaskCommand) -> Result<(), ()> {
        let id = TaskId::from(command.id);
        let _ = self.repository.delete(&id);

        Ok(())
    }
}
