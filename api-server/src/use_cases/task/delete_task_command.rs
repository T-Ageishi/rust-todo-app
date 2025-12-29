use uuid::Uuid;

pub enum DeleteTaskCommandError {
    InvalidFormatTaskId,
}

pub struct DeleteTaskCommand {
    pub id: Uuid,
}

impl DeleteTaskCommand {
    pub fn new(id: &str) -> Result<Self, DeleteTaskCommandError> {
        let id = match Uuid::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Err(DeleteTaskCommandError::InvalidFormatTaskId),
        };

        Ok(Self { id })
    }
}
