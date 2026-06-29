use async_trait::async_trait;
use std::fmt::Debug;

use crate::error::HyraxError;

pub type Result<T> = std::result::Result<T, HyraxError>;

/// A single value produced by a query.
///
/// These variants cover the types most data sources return.
/// The connection implementation is responsible for mapping
/// native types into these variants.
#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

/// Metadata about a column in a query result set.
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
    /// The query returned rows (e.g. SELECT).
    Rows {
        columns: Vec<ColumnInfo>,
        data: Vec<Vec<Value>>,
    },
    /// The query did not return rows (e.g. INSERT, UPDATE, DELETE).
    Executed { rows_affected: u64 },
}

/// Generic interface to any data-source connection.
///
/// Every supported backend (PostgreSQL, MySQL, SQLite, MongoDB,
/// Redis, REST API, …) implements this trait.  Commands never
/// depend on concrete connection types — they only see this
/// trait, which makes the system extensible by adding new
/// implementations.
///
/// # Extending
///
/// 1. Create a new module under `src/connection/`.
/// 2. Implement `Connection` for your type.
/// 3. Add a variant to `ConnectionConfig` in `factory.rs`.
/// 4. Wire the config in `ConnectionFactory::connect()`.
#[async_trait]
pub trait Connection: Debug + Send + Sync {
    /// Returns a short, stable identifier for this connection flavour
    /// (e.g. `"postgresql"`, `"sqlite"`, `"mongodb"`).
    fn connection_type(&self) -> &str;

    /// Lists the top-level relation (table, collection, endpoint, …)
    /// names exposed by the connected data source.
    async fn list_relations(&self) -> Result<Vec<String>>;

    /// Sends a raw query string to the data source and returns the
    /// structured result.
    ///
    /// The interpretation of `query` is backend-specific:
    /// - **SQL** – a SQL string (SELECT, INSERT, …).
    /// - **MongoDB** – a JSON pipeline (future).
    /// - **REST** – an endpoint path (future).
    async fn execute_raw_query(&self, query: &str) -> Result<QueryResult>;
}

pub mod factory;
pub mod sql;

// Re-exports for a cleaner public API.
pub use factory::{ConnectionConfig, ConnectionFactory};
