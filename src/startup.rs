use crate::{
    config::{AdminSettings, SubscribedSettings},
    data::ApplicationData,
    routes,
};
use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;

pub async fn run(
    listener: TcpListener,
    pool: Pool<Postgres>,
    admin: AdminSettings,
    subscribed_settings: SubscribedSettings,
) -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Minimail v0.1.0" }))
        .route("/api/subscribers", get(routes::get_subscribers))
        .route("/api/subscribers", delete(routes::delete))
        .route("/api/subscribe", post(routes::subscribe))
        .with_state(ApplicationData {
            admin,
            pool,
            subscribed: subscribed_settings,
        });

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
