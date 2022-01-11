use axum::{http::Method, routing::get, AddExtensionLayer, Router};
use sqlx::PgPool;
use tower_http::cors::{CorsLayer, Origin};

use portfolio::configuration::get_configuration;
use portfolio::routes::{about::about, career::careers, skill::skills};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let app = Router::new()
        .route("/skills", get(skills))
        .route("/about", get(about))
        .route("/careers", get(careers))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:8000".parse().unwrap()))
                .allow_methods(vec![Method::GET]),
        )
        .layer(AddExtensionLayer::new(connection_pool));

    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
