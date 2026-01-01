const TASK_DESCRIPTION_MAX_LENGTH: usize = 256;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskDescriptionParseError {
    Empty,
    TooLong,
}

#[derive(Debug, Clone)]
pub struct TaskDescription {
    value: String,
}

impl TryFrom<&str> for TaskDescription {
    type Error = TaskDescriptionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.is_empty() {
            return Err(TaskDescriptionParseError::Empty);
        }
        if value.len() > TASK_DESCRIPTION_MAX_LENGTH {
            return Err(TaskDescriptionParseError::TooLong);
        }

        Ok(Self {
            value: String::from(value),
        })
    }
}

impl TaskDescription {
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_when_valid_value_then_returns_instance() {
        let value = "Task Description";
        let task_description = TaskDescription::try_from(value).unwrap();
        assert_eq!(task_description.value, value);

        let value = String::from("A").repeat(TASK_DESCRIPTION_MAX_LENGTH);
        let task_description = TaskDescription::try_from(value.as_str()).unwrap();
        assert_eq!(task_description.value, value);
    }

    #[test]
    fn try_from_when_value_is_empty_then_returns_error() {
        let err = TaskDescription::try_from("").unwrap_err();
        assert_eq!(err, TaskDescriptionParseError::Empty);
    }

    #[test]
    fn _try_from_when_value_is_too_long_then_returns_error() {
        let value = String::from("A").repeat(TASK_DESCRIPTION_MAX_LENGTH + 1);
        let err = TaskDescription::try_from(value.as_str()).unwrap_err();
        assert_eq!(err, TaskDescriptionParseError::TooLong);
    }
}
