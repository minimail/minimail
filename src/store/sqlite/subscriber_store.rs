use sqlx::sqlite::SqlitePool;

use crate::{
    model::{Email, NewSubscriber, Subscriber},
    store::SubscriberStore,
};

pub struct SqliteSubscriberStore {
    pool: SqlitePool,
}

impl SubscriberStore for SqliteSubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Subscriber {
        let row = sqlx::query!(
            r#"
            INSERT INTO subscribers(email)
            VALUES ($1)
            ON CONFLICT(email) DO UPDATE SET id = id
            RETURNING id as 'id!', email as 'email!'
            "#,
            new_subscriber.email.0,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Subscriber {
            id: row.id,
            email: Email::from(row.email),
        }
    }

    async fn all(&self) -> Vec<Subscriber> {
        sqlx::query!("SELECT * FROM SUBSCRIBERS")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Subscriber {
                id: row.id,
                email: Email::from(row.email),
            })
            .collect()
    }

    async fn delete(&mut self, id: i64) -> () {
        sqlx::query!("DELETE FROM subscribers WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Pool, Sqlite};

    use super::*;

    #[tokio::test]
    async fn psql_create_returns_subscriber() {
        let pool = get_pool().await;
        sqlx::migrate!().run(&pool).await.unwrap();

        let mut store = SqliteSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await;

        assert_eq!("test@email.com", subscriber.email.0);
    }

    #[tokio::test]
    async fn create_does_not_duplicate() {
        let pool = get_pool().await;
        let mut store = SqliteSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let initial = store.create(new_subscriber.clone()).await;
        let duplicate = store.create(new_subscriber.clone()).await;

        assert_eq!(initial.id, duplicate.id);
    }

    #[tokio::test]
    async fn delete_removes_subscriber() {
        let pool = get_pool().await;
        let mut store = SqliteSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await;
        store.delete(subscriber.id).await;

        assert!(!store.all().await.iter().any(|s| s.id == subscriber.id));
    }

    #[tokio::test]
    async fn all_lists_subscribers() {
        let pool = get_pool().await;
        let mut store = SqliteSubscriberStore { pool };
        let first_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };
        let second_subscriber = NewSubscriber {
            email: Email::from("another_test@email.com"),
        };

        store.create(first_subscriber).await;
        store.create(second_subscriber).await;
        let subscribers = store.all().await;

        assert!(subscribers.len() >= 2);
    }

    async fn get_pool() -> Pool<Sqlite> {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        pool
    }
}
