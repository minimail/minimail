mod memory;
mod postgres;

pub use memory::InMemorySubscriberStore;
pub use postgres::PsqlSubscriberStore;

use anyhow::Result;

use crate::model::NewSubscriber;
use crate::model::Subscriber;

pub trait SubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Result<Subscriber>;
    async fn all(&self) -> Result<Vec<Subscriber>>;
    async fn delete(&mut self, id: i32) -> Result<()>;
}
