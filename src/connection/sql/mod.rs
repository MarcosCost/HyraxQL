use async_trait::async_trait;
use sqlx::any::AnyRow;
use sqlx::{AnyPool, AssertSqlSafe, Column, Row};

use crate::connection::Result;
use crate::connection::{ColumnInfo, Connection, QueryResult, Value};
use crate::error::HyraxError;

pub mod url;

/// A connection to a SQL database via the sqlx `Any` driver.
///
/// Supports PostgreSQL, MySQL, MariaDB, and SQLite through a single
/// implementation.  The `connection_type` field distinguishes which
/// SQL flavour is actually behind the pool.
///
/// # Thread safety
///
/// `AnyPool` is internally reference-counted and fully `Send + Sync`,
/// so `SqlConnection` can be shared across tasks.
#[derive(Debug)]
pub struct SqlConnection {
    pool: AnyPool,
    connection_type: String,
}

impl SqlConnection {
    pub fn new(pool: AnyPool, connection_type: impl Into<String>) -> Self {
        Self {
            pool,
            connection_type: connection_type.into(),
        }
    }
}

#[async_trait]
impl Connection for SqlConnection {
    fn connection_type(&self) -> &str {
        &self.connection_type
    }

    /// Lists all user-facing tables in the current database.
    ///
    /// The SQL query is selected based on the backend because each
    /// database stores table metadata differently.
    async fn list_relations(&self) -> Result<Vec<String>> {
        // Acquire a temporary connection to inspect the backend name.
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        let backend = conn.backend_name().to_lowercase();
        drop(conn); // release so we can use the pool for the real query

        let query = match backend.as_str() {
            "postgresql" => {
                "SELECT table_name::text FROM information_schema.tables \
                 WHERE table_schema = 'public'"
            }
            "mysql" | "mariadb" => {
                "SELECT CAST(table_name AS CHAR) FROM information_schema.tables \
                 WHERE table_schema = DATABASE()"
            }
            "sqlite" => {
                "SELECT CAST(name AS TEXT) FROM sqlite_master \
                 WHERE type='table' AND name NOT LIKE 'sqlite_%'"
            }
            other => {
                return Err(HyraxError::ConnectionError(format!(
                    "Unsupported SQL backend: {other}",
                )));
            }
        };

        // Static string literal — safe to pass directly.
        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        let names: Vec<String> = rows.iter().map(|row| row.get::<String, _>(0)).collect();
        Ok(names)
    }

    /// Executes an arbitrary SQL string and returns a structured result.
    ///
    /// - For `SELECT` / `WITH` / `RETURNING` queries the result contains
    ///   column metadata and row data.
    /// - DML statements (INSERT without RETURNING, UPDATE, DELETE) that
    ///   return no rows produce `QueryResult::Executed`.
    async fn execute_raw_query(&self, raw: &str) -> Result<QueryResult> {
        let trimmed = raw.trim();

        // Heuristic: if the statement looks like a query that could
        // return rows, use `fetch_all`.  Otherwise use `execute`.
        let upper = trimmed.to_uppercase();
        let returns_rows = upper.starts_with("SELECT")
            || upper.starts_with("WITH")
            || upper.starts_with("EXPLAIN")
            || upper.starts_with("DESCRIBE")
            || upper.starts_with("SHOW");

        if returns_rows {
            self.fetch_rows(trimmed).await
        } else {
            self.execute_dml(trimmed).await
        }
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

impl SqlConnection {
    async fn fetch_rows(&self, query: &str) -> Result<QueryResult> {
        let rows = sqlx::query(AssertSqlSafe(query.to_owned()))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        if rows.is_empty() {
            return Ok(QueryResult::Executed { rows_affected: 0 });
        }

        let columns: Vec<ColumnInfo> = rows[0]
            .columns()
            .iter()
            .map(|col| ColumnInfo {
                name: col.name().to_string(),
                data_type: col.type_info().to_string(),
            })
            .collect();

        let data: Vec<Vec<Value>> = rows
            .iter()
            .map(|row| {
                columns
                    .iter()
                    .enumerate()
                    .map(|(i, _col)| extract_value(row, i))
                    .collect()
            })
            .collect();

        Ok(QueryResult::Rows { columns, data })
    }

    async fn execute_dml(&self, query: &str) -> Result<QueryResult> {
        let result = sqlx::query(AssertSqlSafe(query.to_owned()))
            .execute(&self.pool)
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        Ok(QueryResult::Executed {
            rows_affected: result.rows_affected(),
        })
    }
}

/// Try to extract a cell value as a typed `Value` by probing
/// common SQL column types.
fn extract_value(row: &AnyRow, idx: usize) -> Value {
    if let Ok(v) = row.try_get::<String, _>(idx) {
        return Value::Text(v);
    }
    if let Ok(v) = row.try_get::<i64, _>(idx) {
        return Value::Integer(v);
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
        return Value::Float(v);
    }
    if let Ok(v) = row.try_get::<bool, _>(idx) {
        return Value::Boolean(v);
    }
    // Fallback: try i32 (common for MySQL INT)
    if let Ok(v) = row.try_get::<i32, _>(idx) {
        return Value::Integer(v as i64);
    }
    Value::Null
}
