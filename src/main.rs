mod db;
mod logging;
mod routes;

use std::net::TcpListener;

use anyhow::Result;
use log::info;

use minimail::logging::setup_logging;
use minimail::startup::run;

fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging("./config/log4rs.yaml")?;

    info!("Booting app");

    let db_url = database_url();
    let pool = db::setup_sqlite(&db_url).await;
    let listener = TcpListener::bind("0.0.0.0:3000")?;

    run(listener, pool).await?;

    Ok(())
}
