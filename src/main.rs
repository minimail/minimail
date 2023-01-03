use std::net::TcpListener;

use anyhow::Result;
use log::info;

use minimail::config::get_configuration;
use minimail::db::setup_db;
use minimail::logging::setup_logging;
use minimail::startup::run;

fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging("./config/log4rs.yaml")?;

    info!("Booting app");

    let configuration = get_configuration().expect("Failed to read configuration.");

    let db_url = database_url();
    let pool = setup_db(&db_url).await;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))
    .expect("Failed to setup TCP listener.");

    run(listener, pool).await?;

    Ok(())
}
