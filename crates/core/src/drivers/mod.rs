// crates/core/src/drivers/mod.rs
use async_trait::async_trait;
mod results;
pub use crate::drivers::results::QueryResult;
use crate::error::HyraxResult;

#[async_trait]
pub trait Driver: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn connection_fields(&self) -> Vec<ConnectionField>;

    async fn connect(&mut self, config: &ConnectionConfig) -> HyraxResult<()>;
    async fn disconnect(&mut self) -> HyraxResult<()>;
    async fn execute(&self, query: &str) -> HyraxResult<QueryResult>;
    async fn schema(&self) -> HyraxResult<SchemaInfo>;
    fn is_connected(&self) -> bool;
}
