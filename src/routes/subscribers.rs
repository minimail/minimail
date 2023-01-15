use crate::{
    data::ApplicationData,
    model::NewSubscriber,
    store::{PsqlSubscriberStore, SubscriberStore},
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization, Origin},
    http::StatusCode,
    response::Redirect,
    Form, TypedHeader,
};
use log::debug;

pub async fn get_subscribers(
    State(data): State<ApplicationData>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> Result<String, (StatusCode, String)> {
    debug!("{authorization:?}");

    let authorized = data.admin.token.eq(authorization.0.token());

    if !authorized {
        return Err((StatusCode::UNAUTHORIZED, "Not authorized".to_string()));
    }

    let store = PsqlSubscriberStore::from(data.pool);
    let subscribers = match store.all().await {
        Ok(it) => it,
        Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
    };

    let emails: Vec<String> = subscribers.into_iter().map(|sub| sub.email.0).collect();
    Ok(emails.join("\n"))
}

pub async fn subscribe(
    State(data): State<ApplicationData>,
    TypedHeader(origin): TypedHeader<Origin>,
    Form(new_subscriber): Form<NewSubscriber>,
) -> Redirect {
    let mut store = PsqlSubscriberStore::from(data.pool);
    store.create(new_subscriber).await.unwrap();
    let redirect_url = data
        .subscribed
        .redirect
        .unwrap_or_else(|| origin.to_string());
    Redirect::to(&redirect_url)
}
