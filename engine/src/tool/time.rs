use time::Date;

use crate::{
    Result,
    tool::{Tool, ToolDefinition},
};

// Get current time in local timestamp
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time;

impl Tool for Time {
    type Input = ();
    type Output = String;
    fn init(_session: &mut crate::chat::ChatSession) -> Self {
        Self
    }
    fn definition() -> super::ToolDefinition<Self::Input, Self::Output> {
        ToolDefinition::new("time", "Get current time")
    }

    async fn call(&mut self, _input: Self::Input) -> Result<Self::Output> {
        todo!()
    }
}
