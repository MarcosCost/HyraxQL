use async_trait::async_trait;
use crate::misc::app_enums::AppError;

/* 
//  An Interface the every connection type must implement. (SQL, NoSQL, ...) 
*/

#[async_trait]
pub trait DbProvider: Send + Sync {
    async fn connect(&self, ) -> Result<u32, AppError>;
    async fn disconnect(&self) -> Result<u32, AppError>;
}