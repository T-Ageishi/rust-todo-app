pub struct RegisterTaskCommand {
    title: String,
    description: String,
}

impl RegisterTaskCommand {
    pub fn new(title: &str, description: &str) -> RegisterTaskCommand {
        RegisterTaskCommand {
            title: String::from(title),
            description: String::from(description),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
