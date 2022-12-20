mod memory;
mod sqlite;

pub use memory::InMemorySubscriberStore;
pub use sqlite::SqliteSubscriberStore;

use anyhow::Result;

use crate::model::NewSubscriber;
use crate::model::Subscriber;

pub trait SubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Result<Subscriber>;
    async fn all(&self) -> Result<Vec<Subscriber>>;
    async fn delete(&mut self, id: i64) -> Result<()>;
}
