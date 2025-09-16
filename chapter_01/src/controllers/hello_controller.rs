use axum::{Json, extract::Path};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct HelloResponse {
    pub message: String,
}

pub async fn hello(Path(name): Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("Hola, {}!", name),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::create_routes;

    use axum::body;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_hello_response_struct() {
        let resp = HelloResponse {
            message: "Hola, Rust!".to_string(),
        };
        assert_eq!(resp.message, "Hola, Rust!");
    }

    #[tokio::test]
    async fn test_hello_function() {
        let name = "Federico".to_string();
        let result = hello(Path(name.clone())).await;
        assert_eq!(
            result.0,
            HelloResponse {
                message: format!("Hola, {}!", name)
            }
        );
    }

    #[tokio::test]
    async fn test_create_routes_hello_endpoint() {
        let app = create_routes();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello/Axum")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = body::to_bytes(response.into_body(), 2048).await.unwrap();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        assert!(body_str.contains("Hola, Axum!"));
    }
}
