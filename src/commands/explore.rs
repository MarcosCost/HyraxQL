use sqlx::Row;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use terminal_size::{terminal_size, Width};

use crate::cli::ExploreArgs;
use crate::colors;

pub async fn explore(args: &ExploreArgs, pool: &Option<sqlx::AnyPool>) {
if let Some(ref_pool) = pool.as_ref() { 
        match &args.table {
            Some(table_name) => {
                println!("TODO: table Specific: {}", table_name);
            }
            None => {
                match tables(ref_pool).await {
                    Ok(all_tables) => {
                        format_tables(&all_tables).await;
                    }, 
                    Err(e) => println!("{}Error{}: {}", colors::RED, colors::RESET, e),
                }
            }
        }
    } else {
        println!("{}Error{}: Database is not connected.", colors::RED, colors::RESET);
    }
}

async fn tables(pool: &sqlx::AnyPool) -> Result<Vec<String>, sqlx::Error> {
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

// Helper Functions

async fn format_tables(tables: &Vec<String>){
    if tables.is_empty() {
        return;
    }

    let term_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80
    };

    let max_name_len = tables.iter().map(|s| s.len()).max().unwrap_or(0);
    let cell_width = max_name_len + 3; 

    let calculated_columns = (term_width / cell_width).max(1);
    let desired_columns = calculated_columns.min(tables.len());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    for chunk in tables.chunks(desired_columns) {
        let mut row_cells = Vec::new();
        for name in chunk {
            row_cells.push(Cell::new(name).fg(Color::Cyan));
        }
        
        if tables.len() > desired_columns {
            while row_cells.len() < desired_columns {
                row_cells.push(Cell::new(""));
            }
        }

        table.add_row(row_cells);
    }

    println!("{table}");
}