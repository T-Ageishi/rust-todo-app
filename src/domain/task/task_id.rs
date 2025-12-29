use uuid::Uuid;

#[derive(Debug)]
pub enum TaskIdParseError {
    InvalidIdString,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct TaskId {
    value: Uuid,
}

impl TryFrom<&str> for TaskId {
    type Error = TaskIdParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match Uuid::parse_str(value) {
            Ok(value) => value,
            Err(_) => return Err(TaskIdParseError::InvalidIdString),
        };

        Ok(Self { value })
    }
}

impl TaskId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from(value: Uuid) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_when_valid_uuid_then_returns_instance() {
        let uuid = Uuid::new_v4();
        let task_id = TaskId::from(uuid);
        assert_eq!(task_id.value, uuid);
    }

    #[test]
    fn try_from_when_invalid_uuid_then_returns_error() {
        let uuid = "sample-invalid-uuid";
        let task_id = TaskId::try_from(uuid.to_string().as_str());
        assert!(task_id.is_err());
    }
}
