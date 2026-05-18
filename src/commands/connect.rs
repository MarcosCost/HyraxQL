use std::time::Duration;
use sqlx::{AnyPool, any::{AnyConnectOptions, AnyPoolOptions}};

use crate::cli::ConnectArgs;
use crate::colors;

// Connect and return the Connection Pool
pub async fn run(args: &ConnectArgs) -> Option<AnyPool> {

    if args.url.is_empty() {
        println!("U must provide a non empty connection URL");
        return None;
    }

    let options = match args.url.parse::<AnyConnectOptions>() {
        Ok(opts) => opts,
        Err(e) => {
            println!("{}Invalid connection string format{}: {}",colors::RED,colors::RESET, e);
            return None;
        }
    };

    println!("{}Attempting connection with a 3-second timeout...{}", colors::GRAY, colors::RESET);

    // Build the pool using explicit settings
    let pool_result = AnyPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5)) 
        .connect_with(options)
        .await;

    match pool_result {
        Ok(p) => {
            Some(p)
        }
        Err(e) => {
            println!("\n{}Connection failed:{} \n  -{}\n{}Exiting...{}",colors::RED,colors::RESET,e,colors::GRAY,colors::RESET);
            None
        }
    }
}