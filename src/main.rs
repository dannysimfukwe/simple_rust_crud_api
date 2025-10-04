mod db;
mod state;
mod models;
mod routes;

use state::AppState;
use routes::items::*;
use crate::db::init_db;
use tokio::net::TcpListener;
use std::{env, net::SocketAddr};
use axum::{Router, routing::{get, post, patch, delete}};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("APP_PORT must be a number");

    // DB connection
    let db = init_db().await.expect("Failed to connect to database");

    let state = AppState { db };

    // Routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/items", get(list_items))
        .route("/items", post(create_item))
        .route("/items/:id", get(get_item))
        .route("/items/:id/update", patch(update_item))
        .route("/items/:id/delete", delete(delete_item))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
