use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hyraxql", version = "0.1.0", about = "A fast and lightweight DB explorer")]
pub struct Cli {
    // Flags for subcommandless
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// The subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Connect(ConnectArgs),
}

#[derive(Parser, Debug)]
enum TuiCommands {
    Explore,
    Status,
    Exit,
}

#[derive(Args)]
pub struct ConnectArgs {
    #[arg(short = 'U', long)]
    pub url: String,
}