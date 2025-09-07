use deadpool_redis::{Pool, redis::{AsyncCommands}};
use crate::errors::AppError;

// Guardar valor en Redis
pub async fn set_value(pool: &Pool, key: &str, value: &str) -> Result<(), AppError> {
    let mut conn = pool.get().await.map_err(|_| AppError::InternalError)?;
    conn.set::<String, String, ()>(key.to_string(), value.to_string())
        .await
        .map_err(|_| AppError::InternalError)?;
    Ok(())
}

// Obtener valor de Redis
pub async fn get_value(pool: &Pool, key: &str) -> Result<Option<String>, AppError> {
    let mut conn = pool.get().await.map_err(|_| AppError::InternalError)?;
    let value: Option<String> = conn.get(key).await.map_err(|_| AppError::InternalError)?;
    Ok(value)
}
