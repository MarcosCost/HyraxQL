/* Getting tables, Getting table headers, Listing x entries*/
use crate::{app_state::{AppState, ManagerData}};
use sqlx::Row;

pub async fn get_relation_names(state: &mut AppState) {
    // Safely get the pool
    let pool_borrow = match state.db_pool.as_ref() {
        Some(pool) => pool,
        None => {
            state.set(ManagerData::CommandError("No DB pool found".to_string()));
            return;
        }
    };

    // Safely acquire connection
    let conn = match pool_borrow.acquire().await {
        Ok(c) => c,
        Err(e) => {
            state.set(ManagerData::CommandError(format!("Failed to acquire connection: {:#?}", e)));
            return;
        }
    };

    let dtype = match conn.driver_type().await {
        Ok(t) => t,
        Err(e) => {
            state.set(ManagerData::CommandError(format!("Failed to get driver type: {:?}", e)));
            return;
        }
    };

    // Determine the query string
    let sql_query = match dtype.to_lowercase().as_str() {
        "postgresql" => {
            "SELECT table_name::text FROM information_schema.tables WHERE table_schema = 'public'"
        }
        "mysql" | "mariadb" => {
            "SELECT CAST(table_name AS CHAR) FROM information_schema.tables WHERE table_schema = DATABASE()"
        }
        "sqlite" => {
            "SELECT CAST(name AS TEXT) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
        }
        _ => {
            state.set(ManagerData::CommandError(format!("Database type not recognized: {}", dtype.as_str())));
            return;
        }
    };

    let query = sqlx::query::<sqlx::Any>(sql_query);
    let mut conn_lock = conn.as_any_connection().lock().unwrap();
    
    match query.fetch_all(&mut **conn_lock).await {
        Ok(rows) => {
            let table_names: Vec<String> = rows
                .iter()
                .map(|row| row.get::<String, _>(0))
                .collect();

            state.set(ManagerData::Tables(table_names));
        }
        Err(e) => {
            state.set(ManagerData::CommandError(format!("Database query failed: {}", e)));
        }
    }
}