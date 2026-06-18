use std::{time::Duration};

use sqlx::{any::{AnyPoolOptions}};

use crate::{app_state::{AppState, ManagerData::CommandError}, db::{database::DbPool, sqlx_impl::{SqlxPoolImpl}}, misc::app_structs::ConnectionArgs};

pub async fn connect(state: &mut AppState, args: ConnectionArgs){
    sqlx::any::install_default_drivers();

    let conn_str = match args.db_type.as_str() {
        "postgres"|"mysql"|"mariadb"|"sqlite" => url_builder(&args),
        _ => {
            state.set(CommandError("Unknown Database".to_owned()));
            return;
        }
    };

    let raw_pool = AnyPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&conn_str)
        .await;

    let wrapped_pool = SqlxPoolImpl::new(raw_pool.unwrap());
    let boxed_pool: Option<Box<dyn DbPool>> = Some(Box::new(wrapped_pool));

    state.db_pool = boxed_pool;
    println!("Sucessfully Connected");
}

fn url_builder(args: &ConnectionArgs) -> String{
    if args.db_type == "sqlite" {
        let mut result = format!("sqlite://{}", args.db_name);
        if args.extra_params.is_some() {
            result.push_str("?");
            for (key,value) in args.extra_params.as_ref().unwrap(){
                result.push_str(format!("{}={}&", key, value).as_ref());
            }
            result = result.strip_suffix("&").unwrap().to_owned();
        }
        result
    } else {
        let mut result = format!("{}://{}:{}@{}:{}/{}", args.db_type, args.db_user, args.db_pass, args.host, args.port, args.db_name);
        if args.extra_params.is_some() {
            result.push_str("?");
            for (key,value) in args.extra_params.as_ref().unwrap(){
                result.push_str(format!("{}={}&", key, value).as_ref());
            }
            result = result.strip_suffix("&").unwrap().to_owned();
        }
        result    }
}


#[cfg(test)]
mod tests {
use std::collections::HashMap;

use super::*;

    #[test]
    fn test_sqlite_sem_parametros() {
        let args = ConnectionArgs {
            db_type: "sqlite".to_string(),
            db_name: "meu_banco.db".to_string(),
            db_user: "".to_string(),
            db_pass: "".to_string(),
            host: "".to_string(),
            port: 0,
            extra_params: None,
        };

        assert_eq!(url_builder(&args), "sqlite://meu_banco.db");
    }

    #[test]
    fn test_sqlite_com_parametros() {
        let mut params = HashMap::new();
        params.insert("mode".to_string(), "memory".to_string());

        let args = ConnectionArgs {
            db_type: "sqlite".to_string(),
            db_name: "test.db".to_string(),
            db_user: "".to_string(),
            db_pass: "".to_string(),
            host: "".to_string(),
            port: 0,
            extra_params: Some(params),
        };

        assert_eq!(url_builder(&args), "sqlite://test.db?mode=memory");
    }

    #[test]
    fn test_postgres_sem_parametros() {
        let args = ConnectionArgs {
            db_type: "postgres".to_string(),
            db_name: "producao".to_string(),
            db_user: "admin".to_string(),
            db_pass: "senha123".to_string(),
            host: "127.0.0.1".to_string(),
            port: 5432,
            extra_params: None,
        };

        // Nota: Removido o ']' extra que estava no fim da string original do teu código
        assert_eq!(url_builder(&args), "postgres://admin:senha123@127.0.0.1:5432/producao");
    }

    #[test]
    fn test_postgres_com_multiplos_parametros() {
        let mut params = HashMap::new();
        params.insert("sslmode".to_string(), "require".to_string());

        let args = ConnectionArgs {
            db_type: "mysql".to_string(),
            db_name: "loja".to_string(),
            db_user: "root".to_string(),
            db_pass: "root".to_string(),
            host: "localhost".to_string(),
            port: 3306,
            extra_params: Some(params),
        };

        assert_eq!(url_builder(&args), "mysql://root:root@localhost:3306/loja?sslmode=require");
    }

    #[test]
    fn test_mysql_com_multiplos_parametros() {
        let mut params = std::collections::HashMap::new();
        params.insert("sslmode".to_string(), "verify-ca".to_string());
        params.insert("charset".to_string(), "utf8mb4".to_string());

        let args = ConnectionArgs {
            db_type: "mysql".to_string(),
            db_name: "clientes".to_string(),
            db_user: "user_mysql".to_string(),
            db_pass: "pass_mysql".to_string(),
            host: "mysql-server".to_string(),
            port: 3306,
            extra_params: Some(params),
        };

        let url = url_builder(&args);

        assert!(url.starts_with("mysql://user_mysql:pass_mysql@mysql-server:3306/clientes?"));
        assert!(!url.ends_with("&"));
        assert!(url.contains("sslmode=verify-ca"));
        assert!(url.contains("charset=utf8mb4"));
        assert!(url.contains('&'));
    }

    #[test]
    fn test_mariadb_com_multiplos_parametros() {
        let mut params = std::collections::HashMap::new();
        params.insert("connectTimeout".to_string(), "10".to_string());
        params.insert("compress".to_string(), "true".to_string());

        let args = ConnectionArgs {
            db_type: "mariadb".to_string(),
            db_name: "stock".to_string(),
            db_user: "maria_admin".to_string(),
            db_pass: "maria_pass".to_string(),
            host: "192.168.1.50".to_string(),
            port: 3307,
            extra_params: Some(params),
        };

        let url = url_builder(&args);

        assert!(url.starts_with("mariadb://maria_admin:maria_pass@192.168.1.50:3307/stock?"));
        assert!(!url.ends_with("&"))  ;      
        assert!(url.contains("connectTimeout=10"));
        assert!(url.contains("compress=true"));
        assert!(url.contains('&'));
    }


}