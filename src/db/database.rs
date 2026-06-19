use std::sync::Mutex;

use async_trait::async_trait;
use crate::misc::app_enums::AppError;

/* 
//  An Interface the every connection type must implement. (SQL, NoSQL, ...) 
*/
#[async_trait]
pub trait DbProvider: Send + Sync {
    async fn connect(&self, ) -> Result<u32, AppError>;
    async fn disconnect(&self) -> Result<u32, AppError>;
    async fn driver_type(&self) -> Result<String,AppError>;
    fn as_any_connection(&self) -> &Mutex<sqlx::pool::PoolConnection<sqlx::Any>>; //get the underlying sqlx conn to use on queries
}

#[async_trait]
pub trait DbPool: Send + Sync {
    async fn acquire(&self) -> Result<Box<dyn DbProvider>, AppError>;
}