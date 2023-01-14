use std::net::TcpListener;

use reqwest::redirect::Policy;
use sqlx::{PgPool, Pool, Postgres};

use minimail::{
    config::{AdminSettings, SubscribedSettings},
    startup::run,
};

pub struct TestApp {
    pub address: String,
    pub pool: Pool<Postgres>,
}

async fn spawn_app(pool: PgPool, admin: AdminSettings, redirect: SubscribedSettings) -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let server = run(listener, pool.clone(), admin, redirect);
    tokio::spawn(server);
    TestApp { address, pool }
}

#[sqlx::test]
async fn subscribe_redirects_to_origin(pool: PgPool) {
    // Arrange
    let app = spawn_app(
        pool,
        AdminSettings {
            token: "admin".to_string(),
        },
        SubscribedSettings::default(),
    )
    .await;
    let client = reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .build()
        .unwrap();
    let body = "email=user%40email.com";

    // Act
    let response = client
        .post(&format!("{}/api/subscribers", &app.address))
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

#[sqlx::test]
async fn subscribe_redirects_to_configured_redirect(pool: PgPool) {
    // Arrange
    let app = spawn_app(
        pool,
        AdminSettings {
            token: "admin".to_string(),
        },
        SubscribedSettings {
            redirect: Some("http://example.com".to_string()),
        },
    )
    .await;
    let client = reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .build()
        .unwrap();
    let body = "email=user%40email.com";

    // Act
    let response = client
        .post(&format!("{}/api/subscribers", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(&303, &response.status().as_u16());
    assert_eq!(
        "http://example.com",
        response
            .headers()
            .get("Location")
            .expect("Location header is missing")
    );
}

#[sqlx::test]
async fn subscribe_persists_email(pool: PgPool) {
    // Arrange
    let app = spawn_app(
        pool,
        AdminSettings {
            token: "admin".to_string(),
        },
        SubscribedSettings::default(),
    )
    .await;
    let client = reqwest::Client::new();
    let body = "email=user%40email.com";

    // Act
    client
        .post(&format!("{}/api/subscribers", &app.address))
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

#[sqlx::test]
async fn subscribe_lists_subscribers(pool: PgPool) {
    // Arrange
    let app = spawn_app(
        pool,
        AdminSettings {
            token: "admin".to_string(),
        },
        SubscribedSettings::default(),
    )
    .await;
    let client = reqwest::Client::new();
    let first_subscriber = "email=user%40email.com";
    let second_subscriber = "email=admin%40email.com";

    // Act
    client
        .post(&format!("{}/api/subscribers", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(first_subscriber)
        .send()
        .await
        .expect("Failed to execute request.");
    client
        .post(&format!("{}/api/subscribers", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("origin", &app.address)
        .body(second_subscriber)
        .send()
        .await
        .expect("Failed to execute request.");
    let subscribers = client
        .get(&format!("{}/api/subscribers", &app.address))
        .bearer_auth("admin")
        .send()
        .await
        .expect("Failed to execute request.")
        .text()
        .await
        .expect("Failed to extract subscriber list from response.");

    // Assert
    assert_eq!(subscribers, "user@email.com\nadmin@email.com");
}

#[sqlx::test]
async fn subscribe_without_auth_asks_for_it(pool: PgPool) {
    // Arrange
    let app = spawn_app(
        pool,
        AdminSettings {
            token: "admin".to_string(),
        },
        SubscribedSettings::default(),
    )
    .await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/subscribers", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 400);
    let response_text = response.text().await.expect("No text in body");
    assert_eq!(response_text, "Header of type `authorization` was missing");
}

#[sqlx::test]
async fn subscribe_with_invalid_auth_fails(pool: PgPool) {
    // Arrange
    let app = spawn_app(
        pool,
        AdminSettings {
            token: "admin".to_string(),
        },
        SubscribedSettings::default(),
    )
    .await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/subscribers", &app.address))
        .bearer_auth("BAD_TOKEN")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 401);
    let response_text = response.text().await.expect("No text in body");
    assert_eq!(response_text, "Not authorized");
}
