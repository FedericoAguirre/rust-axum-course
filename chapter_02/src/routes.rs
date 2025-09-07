use crate::controllers::{hello_controller, redis_controller};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello/{name}", get(hello_controller::hello))
        .route("/get/{key}", get(redis_controller::get_key))
        .route("/set/{key}", post(redis_controller::set_key))
}
