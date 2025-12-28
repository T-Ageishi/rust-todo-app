use uuid::Uuid;

#[derive(Debug)]
pub enum UpdateTaskCommandError {
    InvalidFormatTaskID,
}

#[derive(Debug)]
pub struct UpdateTaskCommand {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>,
}

impl UpdateTaskCommand {
    pub fn new(
        id: &str,
        title: Option<&str>,
        description: Option<&str>,
        status: Option<i32>,
    ) -> Result<UpdateTaskCommand, UpdateTaskCommandError> {
        let id = match Uuid::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Err(UpdateTaskCommandError::InvalidFormatTaskID),
        };

        Ok(UpdateTaskCommand {
            id,
            title: match title {
                Some(title) => Some(String::from(title)),
                None => None,
            },
            description: match description {
                Some(description) => Some(String::from(description)),
                None => None,
            },
            status: match status {
                Some(status) => Some(status),
                None => None,
            },
        })
    }
}
