pub struct RegisterTaskCommand {
    title: String,
    description: String,
    status: i32,
}

impl RegisterTaskCommand {
    pub fn new(title: &str, description: &str, status: i32) -> RegisterTaskCommand {
        RegisterTaskCommand {
            title: String::from(title),
            description: String::from(description),
            status,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> i32 {
        self.status
    }
}
