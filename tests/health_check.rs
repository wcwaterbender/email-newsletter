//! tests/health_check.rs
use email_newsletter::configuration::get_configuration;
use sqlx::{Connection, PgConnection};

fn spawn_app() -> String {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscriber_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to load config");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Database");
    let client = reqwest::Client::new();

    let body = "name=sam%20cornish&email=totally_real_email69%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "totally_real_email69@gmail.com");
    assert_eq!(saved.name, "sam cornish");
}

#[actix_web::test]
async fn subscriber_returns_400_when_data_is_malformed() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=sam%20cornish", "missing the email"),
        ("email=totally_real_email69%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (malformed_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(malformed_body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
