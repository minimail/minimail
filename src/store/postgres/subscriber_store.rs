use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};

use crate::{
    model::{Email, NewSubscriber, Subscriber},
    store::SubscriberStore,
};

pub struct PsqlSubscriberStore {
    pool: Pool<Postgres>,
}

impl From<PgPool> for PsqlSubscriberStore {
    fn from(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SubscriberStore for PsqlSubscriberStore {
    async fn create(&mut self, new_subscriber: NewSubscriber) -> Result<Subscriber> {
        let row = sqlx::query!(
            r#"
            INSERT INTO subscribers(email)
            VALUES ($1)
            ON CONFLICT (email) DO UPDATE SET email = EXCLUDED.email
            RETURNING *
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
                id: row.id.into(),
                email: Email::from(row.email),
            })
            .collect())
    }

    async fn delete(&mut self, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM subscribers WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn psql_create_returns_subscriber(pool: PgPool) -> Result<()> {
        let mut store = PsqlSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await?;

        assert_eq!("test@email.com", subscriber.email.0);

        Ok(())
    }

    #[sqlx::test]
    async fn create_does_not_duplicate(pool: PgPool) -> Result<()> {
        let mut store = PsqlSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let initial = store.create(new_subscriber.clone()).await?;
        let duplicate = store.create(new_subscriber.clone()).await?;

        assert_eq!(initial.id, duplicate.id);

        Ok(())
    }

    #[sqlx::test]
    async fn delete_removes_subscriber(pool: PgPool) -> Result<()> {
        let mut store = PsqlSubscriberStore { pool };
        let new_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };

        let subscriber = store.create(new_subscriber).await?;
        store.delete(subscriber.id).await?;

        assert!(!store.all().await?.iter().any(|s| s.id == subscriber.id));

        Ok(())
    }

    #[sqlx::test]
    async fn all_lists_subscribers(pool: PgPool) -> Result<()> {
        let mut store = PsqlSubscriberStore { pool };
        let first_subscriber = NewSubscriber {
            email: Email::from("test@email.com"),
        };
        let second_subscriber = NewSubscriber {
            email: Email::from("another_test@email.com"),
        };

        store.create(first_subscriber).await?;
        store.create(second_subscriber).await?;
        let subscribers = store.all().await?;

        assert!(subscribers.len() == 2);

        Ok(())
    }
}
