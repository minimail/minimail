use crate::{
    auth::get_admin_token,
    model::NewSubscriber,
    store::{SqliteSubscriberStore, SubscriberStore},
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, Origin},
    http::StatusCode,
    response::Redirect,
    Form, TypedHeader,
};
use log::debug;
use sqlx::SqlitePool;

pub async fn subscriber(
    State(pool): State<SqlitePool>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> Result<String, (StatusCode, String)> {
    debug!("{authorization:?}");

    let authorized = get_admin_token()
        .map(|token| token.eq(authorization.0.token()))
        .unwrap_or(true);

    if !authorized {
        return Err((StatusCode::UNAUTHORIZED, "Not authorized".to_string()));
    }

    let store = SqliteSubscriberStore::from(pool);
    let subscribers = match store.all().await {
        Ok(it) => it,
        Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
    };

    let emails: Vec<String> = subscribers.into_iter().map(|sub| sub.email.0).collect();
    Ok(emails.join("\n"))
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
