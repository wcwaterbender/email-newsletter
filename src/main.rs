use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::Application;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //setup our subscriber
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    //import our configuration from configuration.yaml
    let configuration = get_configuration().expect("Failed to read Config");

    //build the server
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
