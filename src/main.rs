mod cli;        // Make rust aware of cli.rs
mod commands;   // Rust automatically searchs for commands/mod.rs
mod colors;

use clap::{CommandFactory, Parser};
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use sqlx::AnyPool;

use crate::cli::Cli;
use crate::cli::Commands;
use crate::cli::TuiCommands;
use crate::commands::explore::*;
use crate::commands::*;

// Autocomplete for TUI functions
struct CommandCompleter;
impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        if pos < line.len() {
            return Ok((0, Vec::new()));
        }

        let cmd = TuiCommands::command();
        let mut candidates = Vec::new();

        let all_commands: Vec<String> = cmd.get_subcommands()
            .map(|c| c.get_name().to_string())
            .chain(std::iter::once("help".to_string()))
            .collect();

        for subcommand in all_commands {
            let name = subcommand;
            if name.starts_with(line) {
                candidates.push(Pair {
                    display: name.to_string(),
                    replacement: name.to_string(),
                });
            }
        }

        Ok((0, candidates))
    }
}
impl Hinter for CommandCompleter {
    type Hint = String;
}
impl Highlighter for CommandCompleter {}
impl Validator for CommandCompleter {}
impl Helper for CommandCompleter {}
 

// =========================================================================
// Main
// =========================================================================

#[tokio::main] // Allow main to be async
async fn main() {

    // Initialization of stuff 
    let cli = Cli::parse();
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

    let mut rl = Editor::<CommandCompleter, rustyline::history::DefaultHistory>::new()
        .expect("Failed to initialize TUI input system");
    rl.set_helper(Some(CommandCompleter));

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

                let _ = rl.add_history_entry(input);

                match parse_tui_command(input) {
                    Ok(command) => match command {
                        TuiCommands::Clear => {
                            print!("\x1b[2J\x1b[1;1H");
                            std::io::Write::flush(&mut std::io::stdout()).unwrap();
                        }
                        TuiCommands::Exit => {
                            if pool.is_some() {println!("Make sure to Disconnect before exiting"); continue;}
                            println!("{}Goodbye!{}", colors::GRAY, colors::RESET);
                            break;
                        }
                        TuiCommands::Connect(args) => {
                            pool = run_connect(&args).await;
                        }
                        TuiCommands::Disconnect => {
                            if let Some(p) = pool.take() {
                                p.close().await;
                            };
                            println!("{}Disconnected!{}", colors::GRAY, colors::RESET);
                        }
                        TuiCommands::Explore(args) => {
                            explore(&args, pool.as_ref()).await;
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

// =========================================================================
// Helpers
// =========================================================================

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

// =========================================================================
// Tests
// =========================================================================

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
