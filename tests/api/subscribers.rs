use std::net::TcpListener;

use reqwest::redirect::Policy;
use sqlx::{Pool, Sqlite};

use minimail::db::setup_sqlite;
use minimail::startup::run;

pub struct TestApp {
    pub address: String,
    pub pool: Pool<Sqlite>,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let pool = setup_sqlite("sqlite::memory:").await;

    let server = run(listener, pool.clone());
    let _ = tokio::spawn(server);
    TestApp { address, pool }
}

#[tokio::test]
async fn subscribe_redirects_to_origin() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .build()
        .unwrap();
    let body = "email=user%40email.com";

    // Act
    let response = client
        .post(&format!("{}/subscriber", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(&303, &response.status().as_u16());
    assert_eq!(
        &app.address,
        response
            .headers()
            .get("Location")
            .expect("Location header is missing")
    );
}

#[tokio::test]
async fn subscribe_persists_email() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "email=user%40email.com";

    // Act
    client
        .post(&format!("{}/subscriber", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    let saved = sqlx::query!("SELECT * FROM subscribers",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscriber.");

    assert_eq!(saved.email, "user@email.com");
}

#[tokio::test]
async fn subscribe_lists_subscribers() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let first_subscriber = "email=user%40email.com";
    let second_subscriber = "email=admin%40email.com";

    // Act
    client
        .post(&format!("{}/subscriber", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(first_subscriber)
        .send()
        .await
        .expect("Failed to execute request.");
    client
        .post(&format!("{}/subscriber", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(second_subscriber)
        .send()
        .await
        .expect("Failed to execute request.");
    let subscribers = client
        .get(&format!("{}/subscriber", &app.address))
        .send()
        .await
        .expect("Failed to execute request.")
        .text()
        .await
        .expect("Failed to extract subscriber list from response.");

    // Assert
    assert_eq!(subscribers, "user@email.com\nadmin@email.com");
}