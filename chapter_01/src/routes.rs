use crate::controllers::hello_controller;
use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new().route("/hello/{name}", get(hello_controller::hello))
}
