use crate::errors::AppError;
use deadpool_redis::{Pool, redis::AsyncTypedCommands};

// Guardar valor en Redis
pub async fn set_value(pool: &Pool, key: &str, value: &str) -> Result<(), AppError> {
    let mut conn = pool.get().await.map_err(|_| AppError::InternalError)?;
    AsyncTypedCommands::set(&mut conn, key.to_string(), value.to_string())
        .await
        .map_err(|_| AppError::InternalError)?;
    Ok(())
}

// Obtener valor de Redis
pub async fn get_value(pool: &Pool, key: &str) -> Result<Option<String>, AppError> {
    let mut conn = pool.get().await.map_err(|_| AppError::InternalError)?;
    let value: Option<String> = AsyncTypedCommands::get(&mut conn, key)
        .await
        .map_err(|_| AppError::InternalError)?;
    Ok(value)
}

pub async fn del_key(pool: &Pool, key: &str) -> Result<(), AppError> {
    let mut conn = pool.get().await.map_err(|_| AppError::InternalError)?;
    AsyncTypedCommands::del(&mut conn, key.to_string())
        .await
        .map_err(|_| AppError::InternalError)?;
    Ok(())
}
