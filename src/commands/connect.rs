use sqlx::{
    AnyPool,
    any::{AnyConnectOptions, AnyPoolOptions},
};
use std::time::Duration;

use crate::cli::ConnectArgs;
use crate::colors;

// Connect and return the Connection Pool
pub async fn run(args: &ConnectArgs) -> Option<AnyPool> {
    let options = match validate_url(&args.build_url()) {
        Ok(opts) => opts,
        Err(err_msg) => {
            println!("{}", err_msg);
            return None;
        }
    };

    println!(
        "{}Attempting connection with a 3-second timeout...{}",
        colors::GRAY,
        colors::RESET
    );

    // Build the pool using explicit settings
    let pool_result = AnyPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await;

    match pool_result {
        Ok(p) => {
            println!("{}Connection Sucessfull!!{}", colors::GRAY, colors::RESET);
            Some(p)
        }
        Err(e) => {
            println!(
                "\n{}Connection failed:{} \n  -{}\n",
                colors::RED,
                colors::RESET,
                e
            );
            None
        }
    }
}

// Validates the connection URL and returns AnyConnectOptions if valid.
fn validate_url(url: &str) -> Result<AnyConnectOptions, String> {
    if url.is_empty() {
        return Err("U must provide a non empty connection URL".to_string());
    }

    url.parse::<AnyConnectOptions>().map_err(|e| {
        format!(
            "{}Invalid connection string format{}: {}",
            colors::RED,
            colors::RESET,
            e
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_empty() {
        let result = validate_url("");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "U must provide a non empty connection URL"
        );
    }

    #[test]
    fn test_validate_url_valid() {
        let result = validate_url("postgres://localhost");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_url_invalid_format() {
        let result = validate_url("not-a-url");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid connection string format")
        );
    }
}
