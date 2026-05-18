mod cli;        // Make rust aware of cli.rs
mod commands;   // Rust automatically searchs for commands/mod.rs

use clap::Parser;
use sqlx::AnyPool;
use std::io::{self, Write};

use crate::cli::Cli;
use crate::cli::Commands;
use crate::commands::run_connect;

#[tokio::main] // Allow main to be async
async fn main() {

    let cli = Cli::parse();
    let mut pool: Option<AnyPool> = None;

    sqlx::any::install_default_drivers();

    if cli.verbose {    // Does nothing
        println!("[DEBUG]: Verbose mode on")
    }

    // Connect from Shell
    if let Some(Commands::Connect(args)) = cli.command {
        println!("Shell input detected. Connecting to {}...", args.url);
        pool = run_connect(&args).await;
        if pool.is_none() {
            return;
        }
    }

    // Fall into the TUI Loop
    println!("--- Interactive DB Explorer ---");
    loop {
        print!("hyraxQL> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Prepare string to be parsable by Clap
        let mut tui_args = vec!["hyraxql"];
        tui_args.extend(input.split_whitespace());

    }

}
