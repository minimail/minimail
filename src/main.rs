mod logging;

use axum::{routing::get, Router};
use log::info;
use logging::setup_logging;

#[tokio::main]
async fn main() {
    setup_logging("./config/log4rs.yaml");

    info!("Booting app");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
