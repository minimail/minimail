use std::collections::HashMap;

use anyhow::Result;
use log::debug;

use crate::{
    model::{Email, NewSubscriber, Subscriber},
    store::SubscriberStore,
};

#[derive(Debug, Default)]
pub struct InMemorySubscriberStore {
    subscribers: HashMap<i32, Subscriber>,
    next_id: i32,
}

impl SubscriberStore for InMemorySubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Result<Subscriber> {
        let existing_subscriber = self
            .subscribers
            .values()
            .find(|s| s.email == new_subscriber.email);

        if let Some(subscriber) = existing_subscriber {
            Ok(subscriber.to_owned())
        } else {
            Ok(self.insert_subscriber(new_subscriber.email))
        }
    }

    async fn all(&self) -> Result<Vec<Subscriber>> {
        Ok(self.subscribers.values().cloned().collect())
    }

    async fn delete(&mut self, email: &Email) -> Result<()> {
        self.subscribers
            .retain(|_key, subscriber| subscriber.email.ne(email));
        Ok(())
    }
}

impl InMemorySubscriberStore {
    fn get_next_id(&mut self) -> i32 {
        self.next_id += 1;
        self.next_id
    }

    fn insert_subscriber(&mut self, email: Email) -> Subscriber {
        let id = self.get_next_id();
        let subscriber = Subscriber { id, email };
        self.subscribers.insert(id, subscriber.clone());
        debug!("subscriber created");
        subscriber
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Email;

    use super::*;

    #[tokio::test]
    async fn create_returns_subscriber() -> Result<()> {
        let mut store = InMemorySubscriberStore::default();
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await?;

        assert_eq!("test@email.com", subscriber.email.0);
        assert_eq!(1, subscriber.id);

        Ok(())
    }

    #[tokio::test]
    async fn create_does_not_duplicate() -> Result<()> {
        let mut store = InMemorySubscriberStore::default();
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        store.create(new_subscriber.clone()).await?;
        store.create(new_subscriber.clone()).await?;
        let subscribers = store.all().await?;

        assert_eq!(1, subscribers.len());

        Ok(())
    }

    #[tokio::test]
    async fn delete_removes_subscriber() -> Result<()> {
        let mut store = InMemorySubscriberStore::default();
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        store.create(new_subscriber).await?;
        store.delete(&Email::from("test@email.com")).await?;
        let subscribers = store.all().await?;

        assert_eq!(0, subscribers.len());

        Ok(())
    }

    #[tokio::test]
    async fn all_lists_subscribers() -> Result<()> {
        let mut store = InMemorySubscriberStore::default();
        let first_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };
        let second_subscriber = NewSubscriber {
            email: Email::from("another_test@email.com"),
        };

        store.create(first_subscriber).await?;
        store.create(second_subscriber).await?;
        let subscribers = store.all().await?;

        assert_eq!(2, subscribers.len());

        Ok(())
    }
}
