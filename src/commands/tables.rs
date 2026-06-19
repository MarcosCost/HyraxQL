/* Getting tables, Getting table headers, Listing x entries*/
use crate::{app_state::{AppState, ManagerData}, misc::app_enums::AppError};
use sqlx::Row;

pub async fn get_relation_names(state: &mut AppState) -> Result<(),AppError> {
    let conn = state.db_pool.as_ref().unwrap().acquire().await?;
    let dtype = conn.driver_type().await?;

    let query = match dtype.to_lowercase().as_str() {
        "postgresql" => {
            // Cast table_name explicitly to TEXT
            sqlx::query::<sqlx::Any>(
                "SELECT table_name::text FROM information_schema.tables WHERE table_schema = 'public'"
            )
        }
        "mysql" | "mariadb" => {
            // MySQL handles text implicitly, but you can use CAST if needed
            sqlx::query::<sqlx::Any>(
                "SELECT CAST(table_name AS CHAR) FROM information_schema.tables WHERE table_schema = DATABASE()"
            )
        }
        "sqlite" => {
            // SQLite uses dynamic typing, standard text works perfectly
            sqlx::query::<sqlx::Any>(
                "SELECT CAST(name AS TEXT) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
            )
        }
        _ => {return Err(AppError::DatabaseError(format!("Database type not recognize: {}",dtype.as_str()).to_owned()));}
    };

    let mut conn_lock = conn.as_any_connection().lock().unwrap();
    let rows = query.fetch_all(&mut **conn_lock)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let table_names: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

    state.set(ManagerData::Tables(table_names));

    Ok(())
}