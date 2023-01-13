use std::net::TcpListener;

use anyhow::Result;
use log::info;

use minimail::config::get_configuration;
use minimail::db::setup_db;
use minimail::logging::setup_logging;
use minimail::startup::run;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging("./config/log4rs.yaml")?;

    info!("Booting app");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = setup_db(&configuration.database.url).await;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))
    .expect("Failed to setup TCP listener.");

    run(listener, pool, configuration.admin).await?;

    Ok(())
}
