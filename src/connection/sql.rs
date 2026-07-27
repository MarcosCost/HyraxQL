use std::ops::Deref;

use async_trait::async_trait;
use sqlx::any::AnyRow;
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
                    "SELECT column_name::text, data_type::text FROM information_schema.columns WHERE table_name = $1;".to_owned()
            }
            "mysql" | "mariadb" => {
                    "SELECT CAST(column_name AS CHAR), CAST(data_type AS CHAR) FROM information_schema.columns WHERE table_name = ?;".to_owned()
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

    async fn get_rows(
        &self,
        sel_tbl: &str,
        size: u32,
        page: u32,
        cols: Vec<String>,
    ) -> Result<Vec<Vec<String>>> {
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;
        let backend = conn.backend_name().to_lowercase();
        drop(conn);

        let mut what: String = "".to_owned();

        let query_str = match backend.as_ref() {
            "postgresql" => {
                if !cols.is_empty() {
                    for i in &cols {
                        what.push_str(format!("{}::text ,", i).deref());
                    }
                    what.pop();
                } else {
                    what = "*".to_owned();
                }
                // result
                format!(
                    "SELECT {} FROM \"{}\" LIMIT $1 OFFSET $2;",
                    what,
                    sel_tbl.replace('"', "\"\"")
                )
            }
            "mysql" | "mariadb" => {
                if !cols.is_empty() {
                    for i in &cols {
                        what.push_str(format!("CAST(`{}` AS CHAR),", i.replace('`', "``")).deref());
                    }
                    what.pop();
                } else {
                    what = "*".to_owned();
                }
                format!(
                    "SELECT {} FROM `{}` LIMIT ? OFFSET ?;",
                    what,
                    sel_tbl.replace('`', "``")
                )
            }
            "sqlite" => {
                if !cols.is_empty() {
                    for i in &cols {
                        what.push_str(format!("\"{}\",", i.replace('"', "\"\"")).deref());
                    }
                    what.pop();
                } else {
                    what = "*".to_owned();
                }
                format!(
                    "SELECT {} FROM \"{}\" LIMIT $1 OFFSET $2;",
                    what,
                    sel_tbl.replace('"', "\"\"")
                )
            }
            other => {
                return Err(HyraxError::ConnectionError(format!(
                    "Unsupported SQL backend: {other}",
                )));
            }
        };

        let rows = sqlx::query(AssertSqlSafe(query_str))
            .bind(size as i64)
            .bind((size * page) as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| HyraxError::QueryError(e.to_string()))?;

        let result = sanitize_rows(&rows);

        Ok(result)
    }
}

///////////////////////////
//     Helpers     //
//////////////////////////

fn sanitize_value(row: &AnyRow, i: usize) -> String {
    if let Ok(v) = row.try_get::<Option<String>, _>(i) {
        return v.unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<i64>, _>(i) {
        return v
            .map(|x| x.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<i32>, _>(i) {
        return v
            .map(|x| x.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<f64>, _>(i) {
        return v
            .map(|x| x.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<f32>, _>(i) {
        return v
            .map(|x| x.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<bool>, _>(i) {
        return v
            .map(|x| x.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(v) = row.try_get::<Option<Vec<u8>>, _>(i) {
        return v
            .map(|bytes| {
                bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            })
            .unwrap_or_else(|| "NULL".to_string());
    }
    "<unsupported>".to_string()
}

fn sanitize_rows(rows: &[AnyRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| (0..row.len()).map(|i| sanitize_value(row, i)).collect())
        .collect()
}
