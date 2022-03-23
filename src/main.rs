use email_newsletter::{configuration::get_configuration, startup::run};
use std::net::TcpListener;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read Config");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
