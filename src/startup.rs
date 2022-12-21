use crate::routes;
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Sqlite};
use std::net::TcpListener;

pub async fn run(listener: TcpListener, pool: Pool<Sqlite>) -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/subscriber", get(routes::subscriber))
        .route("/subscriber", post(routes::create_subscriber))
        .with_state(pool);

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
