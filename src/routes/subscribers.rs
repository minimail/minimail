use crate::{
    model::NewSubscriber,
    store::{SqliteSubscriberStore, SubscriberStore},
};
use axum::{extract::State, headers::Origin, response::Redirect, Form, TypedHeader};
use sqlx::SqlitePool;

pub async fn subscriber(State(pool): State<SqlitePool>) -> String {
    let store = SqliteSubscriberStore::from(pool);

    let emails: Vec<String> = store
        .all()
        .await
        .ok()
        .into_iter()
        .flatten()
        .into_iter()
        .map(|sub| sub.email.0)
        .collect();
    emails.join("\n")
}

pub async fn create_subscriber(
    State(pool): State<SqlitePool>,
    TypedHeader(origin): TypedHeader<Origin>,
    Form(new_subscriber): Form<NewSubscriber>,
) -> Redirect {
    let mut store = SqliteSubscriberStore::from(pool);
    store.create(new_subscriber).await.unwrap();
    Redirect::to(&origin.to_string())
}
