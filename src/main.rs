use axum::{
    http::Method,
    Router,
    routing::{get},
};
use tower_http::cors::{CorsLayer, Origin};
use portfolio::routes::skills::skills;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/skills", get(skills)).layer(
        CorsLayer::new()
            .allow_origin(Origin::exact("http://localhost:8000".parse().unwrap()))
            .allow_methods(vec![Method::GET])
    );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();    
}