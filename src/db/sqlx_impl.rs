use async_trait::async_trait;
use sqlx::{AnyPool, Row, Column};
use crate::db::database::DbProvider;
use crate::app_state::ManagerData;
use crate::misc::app_enums::AppError;


pub struct SqlxBackend {
    pool: AnyPool,
}
impl SqlxBackend {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DbProvider for SqlxBackend {
    async fn connect(&self) -> Result<u32, AppError>{
        Ok(1)
    }
    async fn disconnect(&self) -> Result<u32, AppError>{
        Ok(1)
    }

}