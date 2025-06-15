mod cloud;
mod edge;

#[cfg(not(all(any(target_os = "macos", target_os = "ios"), target_arch = "aarch64")))]
mod llama_cpp;
#[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "aarch64"))]
mod mlx;

use futures_lite::{Stream, StreamExt};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::to_string;

use crate::chat::Message;

pub trait LLM {
    fn respond(&mut self, messages: Vec<Message>) -> impl Stream<Item = String> + Send + Sync;
}

pub struct CategorizeItem<T: Serialize + DeserializeOwned> {
    value: T,
    description: String,
}

pub trait LLMExt: LLM {
    fn oneshot(&mut self, system: &str, user: &str) -> impl Stream<Item = String> + Send + Sync {
        let messages = vec![Message::system(system), Message::user(user)];
        self.respond(messages)
    }

    fn summarize(&mut self, text: &str) -> impl Stream<Item = String> + Send + Sync {
        self.oneshot("Summarize text:", text)
    }

    fn categorize<T: Serialize + DeserializeOwned>(
        &mut self,
        items: Vec<CategorizeItem<T>>,
        text: &str,
    ) -> impl Future<Output = Option<T>> {
        let label_descriptions = items
            .iter()
            .map(|item| {
                let value_name = to_string(&item.value).unwrap();
                let value_name = value_name.trim_matches('"');
                let description = &item.description;
                format!("{value_name}: {description}")
            })
            .collect::<Vec<_>>();
        let label_descriptions = label_descriptions.join("\n");
        let setup_prompt = "You are a helpful AI trained to classify text into predefined categories. Your task: Given a piece of text, determine the most appropriate label from the following categories:";

        let format_prompt = "Output format: Only return the label name.";

        let prompt = format!("{setup_prompt}\n{label_descriptions}\n{format_prompt}");

        async move {
            let mut retry_count = 0;

            loop {
                if retry_count > 3 {
                    return None;
                }

                let value_name = self.oneshot(&prompt, text).collect::<String>().await;
                let value = format!(r#""{value_name}""#);
                if let Ok(value) = serde_json::from_str(&value) {
                    return Some(value);
                } else {
                    retry_count += 1;
                }
            }
        }
    }
}

pub struct OnDeviceModel {}

impl OnDeviceModel {
    pub async fn load() -> Self {
        todo!()
    }

    pub fn is_available() -> bool {
        true
    }
}
