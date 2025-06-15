pub mod code;
pub mod image;
pub mod mem;
pub mod search;
pub mod time;
use crate::Result;
use crate::chat::ChatSession;
use futures_lite::stream::Boxed as BoxedStream;
use futures_lite::{Stream, future::Boxed as BoxedFuture};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{marker::PhantomData, pin::Pin};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition<Input: JsonSchema + DeserializeOwned = (), Output: Serialize = String> {
    name: String,
    description: String,
    _marker: PhantomData<(Input, Output)>,
}

impl<Input: JsonSchema + DeserializeOwned, Output: Serialize> ToolDefinition<Input, Output> {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    pub const fn description(&self) -> &str {
        self.description.as_str()
    }
}

pub type BoxedTextStream = BoxedStream<String>;

pub trait Tool: 'static + Send + Sync {
    type Input: JsonSchema + DeserializeOwned + Send;
    type Output: Serialize;
    fn init(session: &mut ChatSession) -> Self;
    fn definition() -> ToolDefinition<Self::Input, Self::Output>;
    fn call(
        &mut self,
        input: Self::Input,
    ) -> impl Future<Output = crate::Result<Self::Output>> + Send + Sync;
}

trait ToolImpl {
    fn name(&self) -> String;
    fn call_inner<'a>(
        &'a mut self,
        input: &str,
    ) -> Pin<Box<dyn Future<Output = crate::Result<String>> + 'a>>;
}

impl<T: Tool> ToolImpl for T {
    fn name(&self) -> String {
        T::definition().name
    }
    fn call_inner<'a>(
        &'a mut self,
        input: &str,
    ) -> Pin<Box<dyn Future<Output = crate::Result<String>> + 'a>> {
        let input = from_str::<T::Input>(input);

        Box::pin(async move {
            let input = input?;
            let output = self.call(input).await?;
            Ok(to_string_pretty(&output)?)
        })
    }
}

#[allow(clippy::type_complexity)]
pub struct AnyTool(Box<dyn ToolImpl>);

impl AnyTool {
    pub fn new(tool: impl Tool) -> Self {
        Self(Box::new(tool))
    }

    pub fn name(&self) -> String {
        self.0.name()
    }
}

impl Debug for AnyTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        f.write_fmt(format_args!("AnyTool<{name}>"))
    }
}

#[derive(Debug)]
pub struct Tools {
    map: HashMap<String, AnyTool>,
}

impl Tools {
    pub fn insert<T: Tool>(&mut self, tool: T) {
        self.map
            .insert(T::definition().name().to_string(), AnyTool::new(tool));
    }

    pub fn delete_by_name(&mut self, name: &str) {
        self.map.remove(name);
    }

    pub fn delete<T: Tool>(&mut self) {
        self.delete_by_name(T::definition().name());
    }

    pub async fn call<T: Tool>(&mut self, input: T::Input) -> Result<T::Output> {
        todo!()
    }

    pub async fn call_by_name(&mut self, name: &str, input: &str) -> Result<String> {
        todo!()
    }
}
