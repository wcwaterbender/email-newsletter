use std::net::TcpListener;

use email_newsletter::run;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();

    run(listener)?.await
}
