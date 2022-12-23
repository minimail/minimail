use std::str::FromStr;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub async fn setup_db(database_url: &str) -> PgPool {
    let connection_options = PgConnectOptions::from_str(database_url).unwrap();

    let pool = PgPoolOptions::new()
        .connect_with(connection_options)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}
