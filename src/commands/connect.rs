use std::time::Duration;

use crate::cli::ConnectArgs;
use sqlx::{AnyPool, any::{AnyConnectOptions, AnyPoolOptions}};

// Connect and return the Connection Pool
pub async fn run(args: &ConnectArgs) -> Option<AnyPool> {

    if args.url.is_empty() {
        println!("U must provide a non empty connection URL");
        return None;
    }

    let options = match args.url.parse::<AnyConnectOptions>() {
        Ok(opts) => opts,
        Err(e) => {
            println!("Invalid connection string format: {}", e);
            return None;
        }
    };

    println!("Attempting connection with a 3-second timeout...");

    // Build the pool using explicit settings
    let pool_result = AnyPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5)) 
        .connect_with(options)
        .await;

    match pool_result {
        Ok(p) => {
            println!("Connected! Launching TUI...");
            Some(p)
        }
        Err(e) => {
            println!("\nConnection failed: \n  -{}\n Exiting...", e);
            None
        }
    }
}