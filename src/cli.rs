use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "hyraxql", version = "0.1.0", about = "A fast and lightweight DB explorer")]
pub struct Cli {
    // Flags for subcommandless
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// The subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Connect(ConnectArgs),
}

#[derive(Parser, Debug)]
pub enum TuiCommands {
    Connect(ConnectArgs),
    Clear,
    Exit,
}

#[derive(Args, Debug)]
pub struct ConnectArgs {
    #[arg(short = 'U', long)]
    pub url: String,
}