use crate::{config::AdminSettings, data::ApplicationData, routes};
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;

pub async fn run(listener: TcpListener, pool: Pool<Postgres>, admin: AdminSettings) -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Minimail v0.1.0" }))
        .route("/api/subscriber", get(routes::subscriber))
        .route("/api/subscriber", post(routes::create_subscriber))
        .with_state(ApplicationData { admin, pool });

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
