use axum::{routing::get, Router};
use dotenv::dotenv;
use rust_be::config::ProdConfig;
use rust_be::db_connect::establish_connection;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let cfg = ProdConfig::from_env().expect("Cannot load env");
    establish_connection(&cfg.postgres.uri, cfg.postgres.max_conn)
        .await
        .expect("Cannot establish connection");

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/cat", get(get_cat));

    let listener = tokio::net::TcpListener::bind(&cfg.web.address)
        .await
        .unwrap();
    println!("Listening on port {}", cfg.web.address);
    axum::serve(listener, app).await.unwrap();
}

async fn get_cat() -> &'static str {
    println!("Meow meow");
    "Meow meows"
}
