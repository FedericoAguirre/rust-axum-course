use crate::errors::AppError;
use crate::models::redis_model;
use axum::{
    Json,
    extract::{Extension, Path},
};
use deadpool_redis::Pool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetValue {
    value: String,
}

// POST /set/{key}
pub async fn set_key(
    Path(key): Path<String>,
    Extension(pool): Extension<Pool>,
    Json(payload): Json<SetValue>,
) -> Result<Json<serde_json::Value>, AppError> {
    redis_model::set_value(&pool, &key, &payload.value).await?;
    Ok(Json(
        serde_json::json!({ "status": "ok", "key": key, "value": payload.value }),
    ))
}

// GET /get/{key}
pub async fn get_key(
    Path(key): Path<String>,
    Extension(pool): Extension<Pool>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(value) = redis_model::get_value(&pool, &key).await? {
        Ok(Json(serde_json::json!({ "key": key, "value": value })))
    } else {
        Err(AppError::NotFound)
    }
}
