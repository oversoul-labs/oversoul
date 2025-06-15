pub mod chat;
pub(crate) mod database;
pub mod model;
pub mod tool;
pub type Result<T> = std::result::Result<T, anyhow::Error>;
