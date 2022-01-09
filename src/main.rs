use axum::{http::Method, routing::get, Router};
use portfolio::routes::{about::about, career::careers, skill::skills};
use tower_http::cors::{CorsLayer, Origin};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/skills", get(skills))
        .route("/about", get(about))
        .route("/careers", get(careers))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:8000".parse().unwrap()))
                .allow_methods(vec![Method::GET]),
        );

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
