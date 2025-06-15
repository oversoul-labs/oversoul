use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

use crate::tool::Tools;

pub type Id = u64;

pub struct Chats {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct ChatInfo {
    id: Id,
    created_at: Date,
}

impl Chats {
    pub fn list() -> Vec<ChatInfo> {
        todo!()
    }

    pub fn open(name: &str) -> Self {
        todo!()
    }
    pub fn get(&self, id: Id) -> Option<Chat> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Chat {
    info: ChatInfo,
    messages: Vec<Message>,
}

#[derive(Debug)]
pub struct ChatSession {
    chat: Chat,
    tools: Tools,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Message {
    id: Id,
    date: OffsetDateTime,
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Message {
            id: 0,
            date: OffsetDateTime::now_utc(),
            role,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Message::new(Role::System, content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Message::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Message::new(Role::Assistant, content)
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Message::new(Role::Tool, content)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Role {
    System,
    Assistant,
    User,
    Tool,
}
