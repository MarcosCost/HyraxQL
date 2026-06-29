use std::collections::HashMap;
use std::ops::Deref;
use std::time::Duration;

use regex::Regex;
use sqlx::any::AnyPoolOptions;

use crate::connection::sql::SqlConnection;
use crate::connection::sql::url::{build_connection_url, build_sqlite_url};
use crate::connection::{Connection, Result};
use crate::error::HyraxError;

#[derive(Debug, Clone)]
pub enum ConnectionConfig {
    None,
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
}

/// Creates `Connection` trait objects from `ConnectionConfig` values.
/// 1. Add a variant to `ConnectionConfig`.
/// 2. Create a struct implementing `Connection`.
/// 3. Add a `match` arm in `connect()`.
pub struct ConnectionFactory;

pub enum ConnectParams {
    Url(String),
    Config(ConnectionConfig),
}

impl ConnectionFactory {
    pub async fn connect(arg: ConnectParams) -> Result<Box<dyn Connection>> {
        let config: ConnectionConfig;

        match arg {
            ConnectParams::Config(conf) => config = conf,
            ConnectParams::Url(url) => {
                let valid_and_name = validate_sql(url.deref());
                if valid_and_name.0 {
                    let pool = create_pool(&url).await?;
                    return Ok(Box::new(SqlConnection::new(pool, valid_and_name.1)));
                } else {
                    return Err(HyraxError::ConnectionError(
                        "Connection URL was invalid".to_string(),
                    ));
                }
            }
        }

        match config {
            ConnectionConfig::None => Err(HyraxError::ConnectionError(
                "Connection type wasn't identified".to_string(),
            )),
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

fn validate_sql(url: &str) -> (bool, String) {
    let re = Regex::new(r"^sqlite://").unwrap();
    if re.is_match(url) {
        return (true, "sqlite".into());
    }
    let re =
        Regex::new(r"^[a-zA-Z]+://[^:@]+:[^:@]+@[^:@/]+(?::\d+)?/[a-zA-Z]+[^\s]*[^/\s]$").unwrap();
    if re.is_match(url) {
        let ofset = url.find(":").unwrap();
        return (true, url[..ofset].into());
    }
    (false, "".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── SQLite ──────────────────────────────────────────────────────────────

    #[test]
    fn test_validate_sqlite_simple_file() {
        let (ok, kind) = validate_sql("sqlite://data.db");
        assert!(ok);
        assert_eq!(kind, "sqlite");
    }

    #[test]
    fn test_validate_sqlite_with_path() {
        let (ok, kind) = validate_sql("sqlite:///home/user/mydb.sqlite");
        assert!(ok);
        assert_eq!(kind, "sqlite");
    }

    #[test]
    fn test_validate_sqlite_in_memory() {
        let (ok, kind) = validate_sql("sqlite://:memory:");
        assert!(ok);
        assert_eq!(kind, "sqlite");
    }

    #[test]
    fn test_validate_sqlite_with_params() {
        let (ok, kind) = validate_sql("sqlite://test.db?mode=memory&cache=shared");
        assert!(ok);
        assert_eq!(kind, "sqlite");
    }

    // ── PostgreSQL ──────────────────────────────────────────────────────────

    #[test]
    fn test_validate_postgres_standard() {
        let (ok, kind) = validate_sql("postgres://admin:secret@localhost:5432/mydb");
        assert!(ok);
        assert_eq!(kind, "postgres");
    }

    #[test]
    fn test_validate_postgres_no_port() {
        let (ok, kind) = validate_sql("postgres://admin:secret@localhost/mydb");
        assert!(ok);
        assert_eq!(kind, "postgres");
    }

    #[test]
    fn test_validate_postgres_with_params() {
        let (ok, kind) = validate_sql("postgres://u:p@host:5432/db?sslmode=require");
        assert!(ok);
        assert_eq!(kind, "postgres");
    }

    #[test]
    fn test_validate_postgres_special_chars_in_password() {
        let (ok, kind) = validate_sql("postgres://admin:p%40ss@dbhost:5432/production");
        assert!(ok);
        assert_eq!(kind, "postgres");
    }

    // ── MySQL ───────────────────────────────────────────────────────────────

    #[test]
    fn test_validate_mysql_standard() {
        let (ok, kind) = validate_sql("mysql://root:root@127.0.0.1:3306/myshop");
        assert!(ok);
        assert_eq!(kind, "mysql");
    }

    #[test]
    fn test_validate_mysql_domain_host() {
        let (ok, kind) = validate_sql("mysql://app:pass@mysql.example.com:3306/staging");
        assert!(ok);
        assert_eq!(kind, "mysql");
    }

    // ── MariaDB ─────────────────────────────────────────────────────────────

    #[test]
    fn test_validate_mariadb_standard() {
        let (ok, kind) = validate_sql("mariadb://maria:maria_pass@192.168.1.50:3307/warehouse");
        assert!(ok);
        assert_eq!(kind, "mariadb");
    }

    #[test]
    fn test_validate_mariadb_with_params() {
        let url = "mariadb://admin:pass@server:3306/db?connectTimeout=10&compress=true";
        let (ok, kind) = validate_sql(url);
        assert!(ok);
        assert_eq!(kind, "mariadb");
    }

    // ── Invalid / edge cases ────────────────────────────────────────────────

    #[test]
    fn test_validate_empty_string() {
        let (ok, _kind) = validate_sql("");
        assert!(!ok);
    }

    #[test]
    fn test_validate_no_scheme() {
        let (ok, _kind) = validate_sql("just_a_string");
        assert!(!ok);
    }

    #[test]
    fn test_validate_missing_password() {
        let (ok, _kind) = validate_sql("postgres://admin@localhost:5432/db");
        assert!(!ok);
    }

    #[test]
    fn test_validate_missing_user() {
        let (ok, _kind) = validate_sql("postgres://:secret@localhost:5432/db");
        assert!(!ok);
    }

    #[test]
    fn test_validate_missing_host() {
        let (ok, _kind) = validate_sql("postgres://u:p@:5432/db");
        assert!(!ok);
    }

    #[test]
    fn test_validate_missing_database() {
        let (ok, _kind) = validate_sql("postgres://u:p@host:5432/");
        assert!(!ok);
    }

    #[test]
    fn test_validate_unsupported_scheme() {
        let (ok, _kind) = validate_sql("mongodb://u:p@host:27017/mydb");
        assert!(ok);
    }

    #[test]
    fn test_validate_whitespace_in_url() {
        let (ok, _kind) = validate_sql("postgres://u:p@host:5432/db name");
        assert!(!ok);
    }

    #[test]
    fn test_validate_sqlite_uppercase_scheme() {
        let (ok, _kind) = validate_sql("SQLITE://data.db");
        assert!(!ok);
    }

    #[test]
    fn test_validate_postgres_uppercase_scheme() {
        let (ok, kind) = validate_sql("POSTGRES://u:p@host:5432/db");
        assert!(ok);
        assert_eq!(kind, "POSTGRES");
    }

    #[test]
    fn test_validate_port_non_numeric() {
        let (ok, _kind) = validate_sql("postgres://u:p@host:abc/db");
        assert!(!ok);
    }

    #[test]
    fn test_validate_trailing_slash() {
        let (ok, _kind) = validate_sql("postgres://u:p@host:5432/db/");
        assert!(!ok);
    }

    #[test]
    fn test_validate_double_at_sign() {
        let (ok, _kind) = validate_sql("postgres://u:p@@host:5432/db");
        assert!(!ok);
    }

    #[test]
    fn test_validate_newline_in_url() {
        let (ok, _kind) = validate_sql("postgres://u:p@host:5432/db\n");
        assert!(!ok);
    }

    #[test]
    fn test_validate_sqlite_empty_path() {
        // ``^sqlite://`` matches even if nothing follows.
        let (ok, kind) = validate_sql("sqlite://");
        assert!(ok);
        assert_eq!(kind, "sqlite");
    }
}
