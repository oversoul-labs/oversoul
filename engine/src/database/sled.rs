use std::{any::type_name, thread};

use rmp_serde::to_vec;
use serde::{Serialize, de::DeserializeOwned};
use sled::Event;

use crate::database::Database;

pub struct Sled {
    database: sled::Db,
}

impl<T: DeserializeOwned + Serialize + Send + Sync> Database<T> for Sled {
    type Id = u64;
    type Error = sled::Error;
    type WatchGuard = sled::Subscriber;
    async fn open(name: &str) -> Result<Self, Self::Error> {
        let type_name = type_name::<T>();
        Ok(Sled {
            database: sled::open(format!("{name}.{type_name}"))?,
        })
    }

    async fn get(&self, id: Self::Id) -> Result<T, Self::Error> {
        todo!()
    }

    async fn insert(&mut self, item: T) -> Result<Self::Id, Self::Error> {
        let id = self.database.generate_id()?;
        self.database
            .insert(id.to_be_bytes(), to_vec(&item).unwrap())?;
        Ok(id)
    }

    async fn update(&mut self, id: Self::Id, item: T) -> Result<(), Self::Error> {
        self.database
            .insert(id.to_be_bytes(), to_vec(&item).unwrap())?;
        Ok(())
    }

    async fn watch(
        &self,
        id: Self::Id,
        watcher: impl FnMut(T) + Send,
    ) -> Result<Self::WatchGuard, Self::Error> {
        let subscriber = self.database.watch_prefix(id.to_be_bytes());

        Ok(subscriber)
    }

    async fn delete(&mut self, id: Self::Id) -> Result<(), Self::Error> {
        self.database.remove(id.to_be_bytes())?;
        Ok(())
    }
}
