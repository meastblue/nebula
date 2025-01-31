mod server;
mod route;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    server::run()
    .await?;
}
