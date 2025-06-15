pub struct Python {}

impl Default for Python {
    fn default() -> Self {
        todo!()
    }
}

pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub return_value: Option<String>,
}

impl Python {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn execute(&self, code: &str) -> ExecutionResult {
        todo!()
    }
}
