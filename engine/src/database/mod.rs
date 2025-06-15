use serde::{Serialize, de::DeserializeOwned};

pub mod sled;
pub mod sqlite;

pub trait Database<T: DeserializeOwned + Serialize + Send + Sync>: Sized {
    type Id;
    type Error;
    type WatchGuard;
    fn open(name: &str) -> impl Future<Output = Result<Self, Self::Error>> + Send;
    fn get(&self, id: Self::Id) -> impl Future<Output = Result<T, Self::Error>> + Send;
    fn insert(&mut self, item: T) -> impl Future<Output = Result<Self::Id, Self::Error>> + Send;
    fn update(
        &mut self,
        id: Self::Id,
        item: T,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn watch(
        &self,
        id: Self::Id,
        watcher: impl FnMut(T) + Send,
    ) -> impl Future<Output = Result<Self::WatchGuard, Self::Error>> + Send;
    fn delete(&mut self, id: Self::Id) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
