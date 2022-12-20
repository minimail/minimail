mod db;
mod logging;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use log::info;
use logging::setup_logging;

fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap()
}

#[tokio::main]
async fn main() {
    setup_logging("./config/log4rs.yaml").unwrap();

    info!("Booting app");

    let db_url = database_url();
    let pool = db::setup_sqlite(&db_url).await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/subscriber", get(routes::subscriber))
        .route("/subscriber", post(routes::create_subscriber))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
