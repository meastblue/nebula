use axum::{
    routing::get,
    Router,
};

use route;

struct Server;

impl Server {
    async fn run() {
        let host = env::var("SERVER_HOST")?;
        let port = env::var("SERVER_PORT")?.parse::<u16>()?;

        let schema = Schema::new(
            graphql::QueryRoot::default(),
            graphql::MutationRoot::default(),
            EmptySubscription,
        );

        let routes = route::configure();

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();
        println!("Server running on http://localhost:3000");
        axum::serve(listener, routes).await?;
    }
}
