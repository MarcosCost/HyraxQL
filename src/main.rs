use std::io::{self, BufRead};

use hyraxql::connection::ConnectionFactory;
use hyraxql::connection::factory::ConnectParams;

#[tokio::main]
async fn main() {
    println!("Enter connection URL:");
    let mut url = String::new();
    io::stdin().lock().read_line(&mut url).unwrap();
    let url = url.trim().to_string();

    let conn = ConnectionFactory::connect(ConnectParams::Url(url))
        .await
        .expect("Failed to connect");

    println!("Connected! Type: {}", conn.connection_type());

    match conn.list_relations().await {
        Ok(names) => {
            if names.is_empty() {
                println!("No relations found.");
            } else {
                println!("Relations:");
                for name in &names {
                    println!("  {name}");
                }
            }
        }
        Err(e) => println!("Error listing relations: {e}"),
    }
}
