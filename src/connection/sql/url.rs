use std::collections::HashMap;

/// Build a connection URL for database types that use the
/// `scheme://user:pass@host:port/dbname?params` format.
///
/// Used for PostgreSQL, MySQL, MariaDB, and similar databases.
pub fn build_connection_url(
    scheme: &str,
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    database: &str,
    extra_params: &HashMap<String, String>,
) -> String {
    let mut url = format!(
        "{}://{}:{}@{}:{}/{}",
        scheme, user, password, host, port, database
    );
    append_params(&mut url, extra_params);
    url
}

/// Build a connection URL for SQLite (`sqlite://path`).
pub fn build_sqlite_url(path: &str, extra_params: &HashMap<String, String>) -> String {
    let mut url = format!("sqlite://{}", path);
    append_params(&mut url, extra_params);
    url
}

fn append_params(url: &mut String, params: &HashMap<String, String>) {
    if params.is_empty() {
        return;
    }
    url.push('?');
    let parts: Vec<String> = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
    url.push_str(&parts.join("&"));
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_sem_parametros() {
        let params = HashMap::new();
        assert_eq!(
            build_sqlite_url("meu_banco.db", &params),
            "sqlite://meu_banco.db"
        );
    }

    #[test]
    fn test_sqlite_com_parametros() {
        let mut params = HashMap::new();
        params.insert("mode".to_string(), "memory".to_string());
        assert_eq!(
            build_sqlite_url("test.db", &params),
            "sqlite://test.db?mode=memory"
        );
    }

    #[test]
    fn test_postgres_sem_parametros() {
        let params = HashMap::new();
        assert_eq!(
            build_connection_url(
                "postgres",
                "127.0.0.1",
                5432,
                "admin",
                "senha123",
                "producao",
                &params
            ),
            "postgres://admin:senha123@127.0.0.1:5432/producao"
        );
    }

    #[test]
    fn test_mysql_com_parametro_unico() {
        let mut params = HashMap::new();
        params.insert("sslmode".to_string(), "require".to_string());
        assert_eq!(
            build_connection_url("mysql", "localhost", 3306, "root", "root", "loja", &params),
            "mysql://root:root@localhost:3306/loja?sslmode=require"
        );
    }

    #[test]
    fn test_mysql_com_multiplos_parametros() {
        let mut params = HashMap::new();
        params.insert("sslmode".to_string(), "verify-ca".to_string());
        params.insert("charset".to_string(), "utf8mb4".to_string());

        let url = build_connection_url(
            "mysql",
            "mysql-server",
            3306,
            "user_mysql",
            "pass_mysql",
            "clientes",
            &params,
        );

        assert!(url.starts_with("mysql://user_mysql:pass_mysql@mysql-server:3306/clientes?"));
        assert!(!url.ends_with("&"));
        assert!(url.contains("sslmode=verify-ca"));
        assert!(url.contains("charset=utf8mb4"));
        assert!(url.contains('&'));
    }

    #[test]
    fn test_mariadb_com_multiplos_parametros() {
        let mut params = HashMap::new();
        params.insert("connectTimeout".to_string(), "10".to_string());
        params.insert("compress".to_string(), "true".to_string());

        let url = build_connection_url(
            "mariadb",
            "192.168.1.50",
            3307,
            "maria_admin",
            "maria_pass",
            "stock",
            &params,
        );

        assert!(url.starts_with("mariadb://maria_admin:maria_pass@192.168.1.50:3307/stock?"));
        assert!(!url.ends_with("&"));
        assert!(url.contains("connectTimeout=10"));
        assert!(url.contains("compress=true"));
        assert!(url.contains('&'));
    }
}
