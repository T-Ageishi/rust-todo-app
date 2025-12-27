#[derive(Debug, PartialEq, Eq)]
pub enum StatusParseError {
    InvalidStatus
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Todo = 1,
    Doing = 2,
    Done = 3,
}

impl TaskStatus {
    pub fn to_int(&self) -> i32 {
        match self {
            TaskStatus::Todo => 1,
            TaskStatus::Doing => 2,
            TaskStatus::Done => 3,
        }
    }
}

impl TryFrom<i32> for TaskStatus {
    type Error = StatusParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TaskStatus::Todo),
            2 => Ok(TaskStatus::Doing),
            3 => Ok(TaskStatus::Done),
            _ => Err(StatusParseError::InvalidStatus),
        }
    }
}

#[test]
fn try_from_when_valid_value_then_returns_instance() {
    let task_status = TaskStatus::try_from(1).unwrap();
    assert_eq!(task_status, TaskStatus::Todo);

    let task_status = TaskStatus::try_from(2).unwrap();
    assert_eq!(task_status, TaskStatus::Doing);

    let task_status = TaskStatus::try_from(3).unwrap();
    assert_eq!(task_status, TaskStatus::Done);
}

#[test]
fn try_from_when_invalid_value_then_returns_error() {
    let err = TaskStatus::try_from(10).unwrap_err();
    assert_eq!(err, StatusParseError::InvalidStatus);
}
