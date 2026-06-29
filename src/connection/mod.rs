use async_trait::async_trait;
use std::fmt::Debug;

use crate::error::HyraxError;

pub type Result<T> = std::result::Result<T, HyraxError>;

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

/// The typed result of executing a query against a connection.
///
/// Commands should match on this enum and convert it into
/// the appropriate `ManagerData` variant for the UI.
#[derive(Debug, Clone)]
pub enum QueryResult {
    Rows {
        columns: Vec<ColumnInfo>,
        data: Vec<Vec<Value>>,
    },
    /// The query did not return rows (e.g. INSERT, UPDATE, DELETE).
    Executed { rows_affected: u64 },
}

/// Generic interface to any data-source connection.
/// 1. Create a new module under `src/connection/`.
/// 2. Implement `Connection` for your type.
/// 3. Add a variant to `ConnectionConfig` in `factory.rs`.
/// 4. Wire the config in `ConnectionFactory::connect()`.
#[async_trait]
pub trait Connection: Debug + Send + Sync {
    fn connection_type(&self) -> &str;
    async fn list_relations(&self) -> Result<Vec<String>>;

    async fn execute_raw_query(&self, query: &str) -> Result<QueryResult>;
}

pub mod factory;
pub mod sql;

pub use factory::{ConnectionConfig, ConnectionFactory};
