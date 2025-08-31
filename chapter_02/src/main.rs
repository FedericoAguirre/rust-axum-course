use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber;
use deadpool_redis::{Config, Runtime};

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

    // 🔹 Configuración de Redis desde .env
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let mut cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).expect("Cannot create Redis pool");


    // Definir router
    let app = routes::create_routes().layer(Extension(pool));

    // Dirección desde .env o default
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
