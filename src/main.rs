mod cli;        // Make rust aware of cli.rs
mod commands;   // Rust automatically searchs for commands/mod.rs
mod colors;

use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use sqlx::AnyPool;

use crate::cli::Cli;
use crate::cli::Commands;
use crate::cli::TuiCommands;
use crate::commands::*;

#[tokio::main] // Allow main to be async
async fn main() {

    let cli = Cli::parse();
    #[allow(unused)]
    let mut pool: Option<AnyPool> = None;

    sqlx::any::install_default_drivers();

    if cli.verbose {    // Does nothing lol
        println!("[DEBUG]: Verbose mode on")
    }

    // Connect from Shell
    if let Some(Commands::Connect(args)) = cli.command {
        println!("{}Shell input detected. \nConnecting to {}...{}",colors::GRAY, args.url, colors::RESET);
        pool = run_connect(&args).await;
        if pool.is_none() {
            println!("{}Exiting...{}",colors::GRAY,colors::RESET);
            return;
        }
    }

    let mut rl = DefaultEditor::new().expect("Failed to initialize TUI input system");

    // Fall into the TUI Loop
    println!("{}--- Interactive DB Explorer ---{}",colors::GREEN,colors::RESET);
    loop {
        // Read line with a built-in prompt string
        let prompt_string = format!("{}{}{} ", colors::CYAN, "hyraxql>", colors::RESET);
        let readline = rl.readline(&prompt_string);
        
        match readline {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() { continue; }

                // Add successfully typed commands to the up/down arrow history!
                let _ = rl.add_history_entry(input);

                // Create a vector of args that Clap can understand as a shell input
                let mut tui_args = vec!["hyraxql"];
                match shell_words::split(input) {
                    Ok(parsed_args) => {
                        tui_args.extend(parsed_args.iter().map(|s| s.as_str()));

                        // Clap parser
                        match TuiCommands::try_parse_from(tui_args) {
                            Ok(command) => match command {
                                TuiCommands::Clear => {
                                    print!("\x1b[2J\x1b[1;1H");
                                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                                }
                                TuiCommands::Exit => {
                                    println!("{}Goodbye!{}", colors::GRAY, colors::RESET);
                                    break;
                                }
                                TuiCommands::Connect(args) => {
                                    run_connect(&args).await;
                                }
                            },
                            Err(err) => println!("{}", err),
                        }

                    }
                    Err(_) => {
                        println!("{}{}Error:{} Invalid quoting or unclosed string context.", colors::BOLD, colors::RED, colors::RESET);
                        continue;
                    }
                }
            },
            // Handle Ctrl+C or Ctrl+D cleanly
            Err(ReadlineError::Interrupted) => {
                println!("{}Interrupted (Ctrl+C). Exiting...{}", colors::RED, colors::RESET);
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("{}EOF (Ctrl+D). Exiting...{}", colors::GRAY, colors::RESET);
                break;
            }
            Err(err) => {
                println!("Error reading input: {:?}", err);
                break;
            }
        }
    }
}
