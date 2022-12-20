use anyhow::Result;
use sqlx::sqlite::SqlitePool;

use crate::{
    model::{Email, NewSubscriber, Subscriber},
    store::SubscriberStore,
};

pub struct SqliteSubscriberStore {
    pool: SqlitePool,
}

impl From<SqlitePool> for SqliteSubscriberStore {
    fn from(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl SubscriberStore for SqliteSubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Result<Subscriber> {
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
        .await?;

        Ok(Subscriber {
            id: row.id,
            email: Email::from(row.email),
        })
    }

    async fn all(&self) -> Result<Vec<Subscriber>> {
        Ok(sqlx::query!("SELECT * FROM SUBSCRIBERS")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| Subscriber {
                id: row.id,
                email: Email::from(row.email),
            })
            .collect())
    }

    async fn delete(&mut self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM subscribers WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{Pool, Sqlite};

    use super::*;

    #[tokio::test]
    async fn psql_create_returns_subscriber() -> Result<()> {
        let pool = get_pool().await?;
        sqlx::migrate!().run(&pool).await?;

        let mut store = SqliteSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await?;

        assert_eq!("test@email.com", subscriber.email.0);

        Ok(())
    }

    #[tokio::test]
    async fn create_does_not_duplicate() -> Result<()> {
        let pool = get_pool().await?;
        let mut store = SqliteSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let initial = store.create(new_subscriber.clone()).await?;
        let duplicate = store.create(new_subscriber.clone()).await?;

        assert_eq!(initial.id, duplicate.id);

        Ok(())
    }

    #[tokio::test]
    async fn delete_removes_subscriber() -> Result<()> {
        let pool = get_pool().await?;
        let mut store = SqliteSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await?;
        store.delete(subscriber.id).await?;

        assert!(!store.all().await?.iter().any(|s| s.id == subscriber.id));

        Ok(())
    }

    #[tokio::test]
    async fn all_lists_subscribers() -> Result<()> {
        let pool = get_pool().await?;
        let mut store = SqliteSubscriberStore { pool };
        let first_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };
        let second_subscriber = NewSubscriber {
            email: Email::from("another_test@email.com"),
        };

        store.create(first_subscriber).await?;
        store.create(second_subscriber).await?;
        let subscribers = store.all().await?;

        assert!(subscribers.len() >= 2);

        Ok(())
    }

    async fn get_pool() -> Result<Pool<Sqlite>> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(pool)
    }
}
