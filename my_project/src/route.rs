use axum::Router;

pub fn configure() -> Router {
    Router::new()
        .route("/", get(|| async { "Nebulis Backend API" }))
        .route("/hello", get(|| async { "Hello, World!" }));
}
