use email_newsletter::configuration::get_configuration;
use email_newsletter::email_client::EmailClient;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //setup our subscriber
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //import our configuration from configuration.yaml
    let configuration = get_configuration().expect("Failed to read Config");

    //create our pool of connections to the database we are using
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    //init the email client
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");

    let timeout = configuration.email_client.timeout();

    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    //ready set go
    run(listener, connection_pool, email_client)?.await
}
