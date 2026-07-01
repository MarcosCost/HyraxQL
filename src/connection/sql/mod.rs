use std::vec;

use async_trait::async_trait;
use sqlx::{AnyPool, AssertSqlSafe, Row};

use crate::connection::Connection;
use crate::connection::Result;
use crate::error::HyraxError;

pub mod url;

/// A connection to a SQL database via the sqlx Anydriver.
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

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        let names: Vec<String> = rows.iter().map(|row| row.get::<String, _>(0)).collect();
        Ok(names)
    }

    async fn list_rel_headers(&self, sel_tbl: &str) -> Result<Vec<(String, String)>> {
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;
        let backend = conn.backend_name().to_lowercase();
        drop(conn); // release so we can use the pool for the real query

        let query = match backend.as_str() {
            "postgresql" => {
                    "SELECT column_name::text, data_type::text FROM information_schema.columns WHERE table_name = '$1';".to_owned()
            }
            "mysql" | "mariadb" => {
                    "SELECT CAST(column_name AS CHAR), CAST(data_type AS CHAR) FROM information_schema.columns WHERE table_name = '?';".to_owned()
            }
            "sqlite" => {
                "PRAGMA table_info($1);".to_owned()
            }
            other => {
                return Err(HyraxError::ConnectionError(format!(
                    "Unsupported SQL backend: {other}",
                )));
            }
        };

        let rows = sqlx::query(AssertSqlSafe(query))
            .bind(sel_tbl)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        let columns: Vec<(String, String)> = rows
            .iter()
            .map(|row| (row.get::<String, _>(0), row.get::<String, _>(1)))
            .collect();

        Ok(columns)
    }

    async fn get_rows(&self, size: u32, cols: Vec<String>) -> Result<Vec<Vec<String>>> {
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;
        let backend = conn.backend_name().to_lowercase();
        drop(conn);

        Ok(vec![vec!["".to_string()]])
    }
}
