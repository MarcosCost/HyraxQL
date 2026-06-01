use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use serde_json::{Value, json};
use sqlx::{Column, Row, any::AnyRow};
use terminal_size::{Width, terminal_size};

use crate::cli::ExploreArgs;
use crate::colors;

pub async fn explore(args: &ExploreArgs, pool: Option<&sqlx::AnyPool>) -> Result<(), sqlx::Error> {
    let Some(ref_pool) = pool else {
        println!(
            "{}Error{}: Database is not connected.",
            colors::RED,
            colors::RESET
        );
        return Ok(());
    };

    // -t
    if let Some(table_name) = &args.table {
        let mut conn = ref_pool.acquire().await?;
        let db_type = conn.backend_name();

        // -t -c
        if args.columns {
            let query = match db_type {
                "PostgreSQL" => {
                    "SELECT column_name::text, data_type::text FROM information_schema.columns WHERE table_name = $1 AND table_schema = 'public'"
                }
                "MySQL" => {
                    "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = ? AND table_schema = DATABASE()"
                }
                "SQLite" => "PRAGMA table_info(?)",
                _ => {
                    return Err(sqlx::Error::Configuration(
                        format!("Unsupported database driver: {}", db_type).into(),
                    ));
                }
            };

            let rows = sqlx::query(query)
                .bind(table_name)
                .fetch_all(&mut *conn)
                .await?;

            let columns: Vec<Vec<(String, Value)>> = rows
                .iter()
                .map(|row: &AnyRow| {
                    row.columns()
                        .iter()
                        .map(|col| (col.name().to_string(), value_to_json(row, col.ordinal())))
                        .collect()
                })
                .collect();

            format_query_results(&columns);
            return Ok(());
        }

        // -t
        let query = match db_type {
            "PostgreSQL" => format!("SELECT * FROM {} LIMIT $1;", table_name),
            "MySQL" => format!("SELECT * FROM {} LIMIT ?", table_name),
            "SQLite" => format!("SELECT * FROM {} LIMIT ?;", table_name),
            _ => {
                return Err(sqlx::Error::Configuration(
                    format!("Unsupported database driver: {}", db_type).into(),
                ));
            }
        };

        let rows = sqlx::query(&query)
            .bind(args.size)
            .fetch_all(&mut *conn)
            .await?;

        let columns: Vec<Vec<(String, Value)>> = rows
            .iter()
            .map(|row: &AnyRow| {
                row.columns()
                    .iter()
                    .map(|col| (col.name().to_string(), value_to_json(row, col.ordinal())))
                    .collect()
            })
            .collect();

        format_query_results(&columns);
        return Ok(());
    }

    // no args, show all tables
    match tables(ref_pool).await {
        Ok(all_tables) => {
            println!("{}Tables:{}", colors::BOLD, colors::RESET);
            format_tables(&all_tables);
            Ok(())
        }
        Err(e) => {
            println!("{}Error{}: {}", colors::RED, colors::RESET, e);
            Err(e)
        }
    }
}

// Get all table Names
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
        _ => {
            return Err(sqlx::Error::Configuration(
                format!("Unsupported database driver: {}", db_type).into(),
            ));
        }
    };

    let rows = sqlx::query(query).fetch_all(&mut *conn).await?;

    rows.iter()
        .map(|row| row.try_get::<String, _>(0))
        .collect::<Result<Vec<String>, _>>()
}

// Helper Functions
fn format_tables(tables: &[String]) {
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
        let mut row_cells: Vec<Cell> = chunk
            .iter()
            .map(|name| Cell::new(name).fg(Color::Cyan))
            .collect();

        if tables.len() > desired_columns {
            while row_cells.len() < desired_columns {
                row_cells.push(Cell::new(""));
            }
        }

        table.add_row(row_cells);
    }

    println!("{table}");
}

fn format_query_results(rows: &Vec<Vec<(String, Value)>>) {
    if rows.is_empty() {
        println!("No results.");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // Header row from first row's keys
    let headers: Vec<Cell> = rows[0]
        .iter()
        .map(|(k, _)| Cell::new(k).fg(Color::Green).add_attribute(Attribute::Bold))
        .collect();
    table.set_header(headers);

    // Data rows
    for row in rows {
        let cells: Vec<Cell> = row
            .iter()
            .map(|(_, v)| {
                let s = match v {
                    Value::String(s) => s.clone(),
                    Value::Null => "NULL".to_string(),
                    other => other.to_string(),
                };
                Cell::new(s).fg(Color::Cyan)
            })
            .collect();
        table.add_row(cells);
    }

    println!("{table}");
}

fn value_to_json(row: &AnyRow, ordinal: usize) -> Value {
    if let Ok(v) = row.try_get::<bool, _>(ordinal) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<i64, _>(ordinal) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<f64, _>(ordinal) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<String, _>(ordinal) {
        return json!(v);
    }
    if let Ok(v) = row.try_get::<Vec<u8>, _>(ordinal) {
        return json!(v);
    }
    json!(null)
}
