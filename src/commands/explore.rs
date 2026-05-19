use sqlx::Row;

pub async fn tables(pool: &sqlx::AnyPool) -> Result<Vec<String>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let db_type = conn.backend_name();
    
    let query = match db_type {
        "PostgreSQL" => {
            "SELECT table_name::text FROM information_schema.tables WHERE table_schema = 'public'"
        }
        "MySQL" => {
            "SELECT table_name FROM information_schema.tables WHERE table_schema = DATABASE()"
        }
        "SQLite" => {
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
        }
        _ => return Err(sqlx::Error::Configuration(
            format!("Unsupported database driver: {}", db_type).into()
        )),
    };
    
    let rows = sqlx::query(query).fetch_all(&mut *conn).await?;

    let tables = rows.iter()
        .map(|row| row.try_get::<String, _>(0).unwrap_or_default())
        .collect();

    Ok(tables)
}