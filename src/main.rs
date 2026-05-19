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
use crate::commands::explore::*;
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
        println!("{}Shell input detected. \nConnecting to {}...{}",colors::GRAY, args.build_url(), colors::RESET);
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

                match parse_tui_command(input) {
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
                            pool = run_connect(&args).await;
                        }
                        TuiCommands::Disconnect => {
                            pool = None;
                            println!("{}Disconnected!{}", colors::GRAY, colors::RESET);
                        }
                        TuiCommands::Explore => {
                            if let Some(ref_pool) = pool.as_ref() {
                                // TODO: Proper visual display
                                println!("{:#?}", tables(ref_pool).await);
                            } else {
                                println!("{}Error{}: Database is not connected.",colors::RED,colors::RESET); 
                            }
                        }
                    },
                    Err(err) => println!("{}", err),
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

/// Parses a TUI input string into a TuiCommands enum.
fn parse_tui_command(input: &str) -> Result<TuiCommands, String> {
    match shell_words::split(input) {
        Ok(parsed_args) => {
            let mut tui_args = vec!["hyraxql"];
            tui_args.extend(parsed_args.iter().map(|s| s.as_str()));

            TuiCommands::try_parse_from(tui_args)
                .map_err(|err| err.to_string())
        }
        Err(_) => Err(format!(
            "{}{}Error:{} Invalid quoting or unclosed string context.",
            colors::BOLD, colors::RED, colors::RESET
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tui_command_valid_exit() {
        let result = parse_tui_command("exit");
        assert!(matches!(result, Ok(TuiCommands::Exit)));
    }

    #[test]
    fn test_parse_tui_command_valid_clear() {
        let result = parse_tui_command("clear");
        assert!(matches!(result, Ok(TuiCommands::Clear)));
    }

    #[test]
    fn test_parse_tui_command_valid_connect() {
        let result = parse_tui_command("connect -t postgres -u marcos -d hyrax_dev -h localhost");
        if let Ok(TuiCommands::Connect(args)) = result {
            assert_eq!(args.build_url(), "postgres://marcos@localhost:5432/hyrax_dev");
        } else {
            panic!("Expected Ok(TuiCommands::Connect), got {:?}", result);
        }
    }

    #[test]
    fn test_parse_tui_command_quoted_url() {
        let result = parse_tui_command("connect -t sqlite -d \"my db.sqlite\"");
        if let Ok(TuiCommands::Connect(args)) = result {
            assert_eq!(args.build_url(), "sqlite://my db.sqlite");
        } else {
            panic!("Expected Ok(TuiCommands::Connect), got {:?}", result);
        }
    }

    #[test]
    fn test_parse_tui_command_extra_spaces() {
        let result = parse_tui_command("connect      -t       postgres       -u      marcos -d       hyrax_dev      -h       localhost");
        if let Ok(TuiCommands::Connect(args)) = result {
            assert_eq!(args.build_url(), "postgres://marcos@localhost:5432/hyrax_dev");
        } else {
            panic!("Expected Ok(TuiCommands::Connect), got {:?}", result);
        }
    }

    #[test]
    fn test_parse_tui_command_invalid_command() {
        let result = parse_tui_command("invalid-cmd");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tui_command_unclosed_quote() {
        let result = parse_tui_command("connect -U \"unclosed quote");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid quoting"));
    }
}
