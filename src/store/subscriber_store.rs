use std::collections::HashMap;

use crate::model::{NewSubscriber, Subscriber};

pub trait SubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Subscriber;
    async fn all(&self) -> Vec<Subscriber>;
    async fn delete(&mut self, id: i32) -> ();
}

#[derive(Debug, Default)]
pub struct InMemorySubscriberStore {
    subscribers: HashMap<i32, Subscriber>,
    next_id: i32,
}

impl SubscriberStore for InMemorySubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Subscriber {
        let id = self.get_next_id();
        let subscriber = Subscriber {
            id,
            email: new_subscriber.email,
        };
        self.subscribers.insert(id, subscriber.clone());
        subscriber
    }

    async fn all(&self) -> Vec<Subscriber> {
        self.subscribers.values().cloned().collect()
    }

    async fn delete(&mut self, id: i32) -> () {
        self.subscribers.remove(&id);
    }
}

impl InMemorySubscriberStore {
    fn get_next_id(&mut self) -> i32 {
        self.next_id += 1;
        self.next_id
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Email;

    use super::*;

    #[tokio::test]
    async fn create_returns_subscriber() {
        let mut store = InMemorySubscriberStore::default();
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await;

        assert_eq!("test@email.com", subscriber.email.0);
        assert_eq!(1, subscriber.id);
    }

    #[tokio::test]
    async fn delete_removes_subscriber() {
        let mut store = InMemorySubscriberStore::default();
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await;
        store.delete(subscriber.id).await;
        let subscribers = store.all().await;

        assert_eq!(0, subscribers.len());
    }

    #[tokio::test]
    async fn all_lists_subscribers() {
        let mut store = InMemorySubscriberStore::default();
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        store.create(new_subscriber.clone()).await;
        store.create(new_subscriber.clone()).await;
        let subscribers = store.all().await;

        assert_eq!(2, subscribers.len());
    }
}
