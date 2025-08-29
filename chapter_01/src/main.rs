use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber;

mod controllers;
mod errors;
mod models;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    // Cargar variables de entorno
    dotenv().ok();

    // Inicializar logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    // Definir router
    let app = routes::create_routes();

    // Dirección desde .env o default
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
