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
    Explore(ExploreArgs), // List all tables in the db, regardless of type, or get a specific table
    Clear,
    Disconnect,
    Exit,
}

#[derive(Args, Debug)]
#[command(disable_help_flag = true)]
pub struct ConnectArgs {
    // postgres, mysql, mariadb, or sqlite are the officially tested
    #[clap(short = 't', long = "type")]
    pub db_type: String,

    #[clap(short = 'u', long = "user", default_value = "sqlite")]
    pub user: String,

    #[clap(short = 'd', long = "db")]
    pub dbname: String,

    #[clap(short = 'h', long = "host", default_value = "localhost")]
    pub host: String,

    #[clap(short = 'p', long = "port", default_value = "5432")]
    pub port: String,

    #[clap(short = 'w', long = "pw")]
    pub password: Option<String>,
}
impl ConnectArgs {
    pub fn build_url(&self) -> String {

        if self.db_type.to_lowercase() == "sqlite" {
            return format!("sqlite://{}", self.dbname);
        }

        match &self.password {
            // type://user:password@host:port/db_name
            Some(pass) => format!(
                "{}://{}:{}@{}:{}/{}",
                self.db_type, self.user, pass, self.host, self.port, self.dbname
            ),
            // type://user@host:port/db_name
            None => format!(
                "{}://{}@{}:{}/{}",
                self.db_type, self.user, self.host, self.port, self.dbname
            ),
        }
    }
}

#[derive(Args, Debug)]
#[command(disable_help_flag = true)]
pub struct ExploreArgs {
    #[clap(short = 't', long = "table")]
    pub table: Option<String>,
}

// ==========
// Unit Tests
// ==========
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_builder_with_password() {
        // 1. Manually craft a fake set of input arguments
        let args = ConnectArgs {
            db_type: "postgres".to_string(),
            user: "marcos".to_string(),
            dbname: "hyrax_prod".to_string(),
            host: "10.0.0.1".to_string(),
            port: "5432".to_string(),
            password: Some("my_secret_pass".to_string())
        };
        let url = args.build_url();

        assert_eq!(url, "postgres://marcos:my_secret_pass@10.0.0.1:5432/hyrax_prod");
    }

    #[test]
    fn test_url_builder_without_password() {
        let args = ConnectArgs {
            db_type: "mysql".to_string(),
            user: "root".to_string(),
            dbname: "dev_db".to_string(),
            host: "localhost".to_string(),
            port: "3306".to_string(),
            password: None, // NO password
        };
        let url = args.build_url();

        assert_eq!(url, "mysql://root@localhost:3306/dev_db");
    }

    #[test]
    fn test_url_builder_for_sqlite() {
        let args = ConnectArgs {
            db_type: "sqlite".to_string(),
            user: "".to_string(), // Omitted for local files
            dbname: "local_store.db".to_string(),
            host: "".to_string(),
            port: "".to_string(),
            password: None,
        };

        let url = args.build_url();

        assert_eq!(url, "sqlite://local_store.db");
    }
}