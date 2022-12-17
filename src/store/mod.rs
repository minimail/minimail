mod memory;

pub use memory::InMemorySubscriberStore;

use crate::model::NewSubscriber;
use crate::model::Subscriber;

pub trait SubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Subscriber;
    async fn all(&self) -> Vec<Subscriber>;
    async fn delete(&mut self, id: i64) -> ();
}
