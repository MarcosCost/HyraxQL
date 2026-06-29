use std::collections::HashMap;
use std::time::Duration;

use sqlx::any::AnyPoolOptions;

use crate::connection::sql::SqlConnection;
use crate::connection::sql::url::{build_connection_url, build_sqlite_url};
use crate::connection::{Connection, Result};
use crate::error::HyraxError;

/// Configuration for establishing a connection.
///
/// Each variant carries only the parameters relevant to that
/// backend.  Add a new variant here when you introduce a new
/// connection type.
#[derive(Debug, Clone)]
pub enum ConnectionConfig {
    Postgres {
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
        extra_params: HashMap<String, String>,
    },
    Mysql {
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
        extra_params: HashMap<String, String>,
    },
    Mariadb {
        host: String,
        port: u16,
        user: String,
        password: String,
        database: String,
        extra_params: HashMap<String, String>,
    },
    Sqlite {
        path: String,
        extra_params: HashMap<String, String>,
    },
    // ---------------------------------------------------------------
    // Future connection types: add a new variant and a match arm in
    // `ConnectionFactory::connect`.
    //
    //     Mongo { .. }
    //     Redis { .. }
    //     RestApi { .. }
    // ---------------------------------------------------------------
}

/// Creates `Connection` trait objects from `ConnectionConfig` values.
///
/// This is the single place where the mapping from configuration to
/// connection implementation lives.  To add a new connection type:
///
/// 1. Add a variant to `ConnectionConfig`.
/// 2. Create a struct implementing `Connection`.
/// 3. Add a `match` arm in `connect()`.
pub struct ConnectionFactory;

impl ConnectionFactory {
    /// Establish a connection based on the provided configuration.
    ///
    /// Returns a boxed `Connection` that the caller (typically the
    /// `Engine`) can use to run commands.
    pub async fn connect(config: ConnectionConfig) -> Result<Box<dyn Connection>> {
        match config {
            ConnectionConfig::Postgres {
                host,
                port,
                user,
                password,
                database,
                extra_params,
            } => {
                let url = build_connection_url(
                    "postgres",
                    &host,
                    port,
                    &user,
                    &password,
                    &database,
                    &extra_params,
                );
                let pool = create_pool(&url).await?;
                Ok(Box::new(SqlConnection::new(pool, "postgresql")))
            }

            ConnectionConfig::Mysql {
                host,
                port,
                user,
                password,
                database,
                extra_params,
            } => {
                let url = build_connection_url(
                    "mysql",
                    &host,
                    port,
                    &user,
                    &password,
                    &database,
                    &extra_params,
                );
                let pool = create_pool(&url).await?;
                Ok(Box::new(SqlConnection::new(pool, "mysql")))
            }

            ConnectionConfig::Mariadb {
                host,
                port,
                user,
                password,
                database,
                extra_params,
            } => {
                let url = build_connection_url(
                    "mariadb",
                    &host,
                    port,
                    &user,
                    &password,
                    &database,
                    &extra_params,
                );
                let pool = create_pool(&url).await?;
                Ok(Box::new(SqlConnection::new(pool, "mariadb")))
            }

            ConnectionConfig::Sqlite { path, extra_params } => {
                let url = build_sqlite_url(&path, &extra_params);
                let pool = create_pool(&url).await?;
                Ok(Box::new(SqlConnection::new(pool, "sqlite")))
            }
        }
    }
}

async fn create_pool(url: &str) -> Result<sqlx::AnyPool> {
    sqlx::any::install_default_drivers();

    AnyPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await
        .map_err(|e| HyraxError::ConnectionError(e.to_string()))
}
