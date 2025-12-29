const TASK_TITLE_MAX_LENGTH: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskTitleParseError {
    Empty,
    TooLong,
}

#[derive(Debug)]
pub struct TaskTitle {
    value: String,
}

impl TryFrom<&str> for TaskTitle {
    type Error = TaskTitleParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(TaskTitleParseError::Empty);
        }
        if value.len() > TASK_TITLE_MAX_LENGTH {
            return Err(TaskTitleParseError::TooLong);
        }

        Ok(TaskTitle {
            value: String::from(value),
        })
    }
}

impl TaskTitle {
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_when_valid_value_then_returns_instance() {
        let value = String::from("Task Title");
        let task_title = TaskTitle::try_from(value.as_str()).unwrap();
        assert_eq!(task_title.value, "Task Title");

        let value = String::from("A").repeat(TASK_TITLE_MAX_LENGTH);
        let task_title = TaskTitle::try_from(value.as_str()).unwrap();
        assert_eq!(task_title.value, value);
    }

    #[test]
    fn try_from_when_value_is_empty_then_returns_error() {
        let err = TaskTitle::try_from("").unwrap_err();
        assert_eq!(err, TaskTitleParseError::Empty);
    }

    #[test]
    fn try_from_when_value_is_too_long_then_returns_error() {
        let value = String::from("A").repeat(TASK_TITLE_MAX_LENGTH + 1);
        let err = TaskTitle::try_from(value.as_str()).unwrap_err();
        assert_eq!(err, TaskTitleParseError::TooLong);
    }
}
