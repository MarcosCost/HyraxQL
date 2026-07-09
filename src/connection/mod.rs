use async_trait::async_trait;
use std::fmt::Debug;

use crate::error::HyraxError;

pub type Result<T> = std::result::Result<T, HyraxError>;

/// Generic interface to any data-source connection.
/// 1. Create a new module under `src/connection/`.
/// 2. Implement `Connection` for your type.
/// 3. Add a variant to `ConnectionConfig` in `factory.rs`.
/// 4. Wire the config in `ConnectionFactory::connect()`.
#[async_trait]
pub trait Connection: Debug + Send + Sync {
    fn connection_type(&self) -> &str;
    async fn list_relations(&self) -> Result<Vec<String>>;
    async fn list_rel_headers(&self, sel_tbl: &str) -> Result<Vec<(String, String)>>;
    async fn get_rows(
        &self,
        sel_tbl: &str,
        size: u32,
        page: u32,
        cols: Vec<String>,
    ) -> Result<Vec<Vec<String>>>;
}

pub mod factory;
pub mod sql;

pub use factory::{ConnectionConfig, ConnectionFactory};
