use axum::{routing::get, Router};
use entity::cat;
use rust_be::db_connect::establish_connection;
use sea_orm::{EntityTrait, Set};

#[tokio::main]
async fn main() {
    let db = establish_connection()
        .await
        .expect("Cannot establish connection");
    let test_cat = cat::ActiveModel {
        name: Set(String::from("Shirou")),
        ..Default::default()
    };

    let result = cat::Entity::insert(test_cat)
        .exec(&db)
        .await
        .expect("Cannot create");

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/cat", get(get_cat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_cat() -> &'static str {
    println!("Meow meow");
    "Meow meows"
}
