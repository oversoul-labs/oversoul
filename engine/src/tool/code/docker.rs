pub struct Docker {}

pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
}

impl Docker {
    pub async fn new() -> Self {
        todo!()
    }

    pub async fn run(&self, shell_code: &str) -> ExecutionResult {
        todo!()
    }

    pub async fn destory(self) {
        todo!()
    }
}
