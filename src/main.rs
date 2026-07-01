use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

use hyraxql::commands::Command;
use hyraxql::commands::list_rel_head::ListHeaders;
use hyraxql::commands::list_tables::ListTables;
use hyraxql::commands::select_table::SelTable;
use hyraxql::connection::ConnectionFactory;
use hyraxql::connection::factory::ConnectParams;
use hyraxql::engine::state::AppState;

///
///
///     THIS MAIN IS A MOCK BINARY. ITS HERE PURELLY SO I CAN RUN THE ENGINE BEFORE BUILDING ANY UI
///
///

#[tokio::main]
async fn main() {
    println!("Enter connection URL:");
    let mut url = String::new();
    io::stdin().lock().read_line(&mut url).unwrap();
    let url = url.trim().to_string();

    let mut tentativa = 1;

    let conn = loop {
        match ConnectionFactory::connect(ConnectParams::Url(url.clone())).await {
            Ok(connection) => {
                println!("Connection established {} trys(s)!", tentativa);
                break connection;
            }
            Err(err) => {
                eprintln!("Trys #{} failed: {}", tentativa, err);
                eprintln!(
                    "Warning. Attempting to reconnect in 3 seconds... (Press Ctrl+C to stop)"
                );

                tentativa += 1;
                sleep(Duration::from_secs(3));
            }
        }
    };

    println!("Connected! Type: {}", conn.connection_type());

    // Set up the command infrastructure.
    let (_tx, _rx) = mpsc::channel();
    let mut state = AppState::new(_tx);

    if let Err(e) = ListTables.execute(&*conn, &mut state).await {
        eprintln!("Error listing relations: {e}");
        return;
    }

    let cmd = SelTable {
        nome: "people".into(),
    };
    if let Err(e) = cmd.execute(&*conn, &mut state).await {
        eprintln!("Error Setting relations: {e}");
        return;
    }

    let cmd = ListHeaders;
    if let Err(e) = cmd.execute(&*conn, &mut state).await {
        eprintln!("Error listing relations: {e}");
        return;
    }
    print!("{:?}", state.current_data)
}
