use axum::{Json, extract::Path};
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn hello(Path(name): Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("Hola, {}!", name),
    })
}
