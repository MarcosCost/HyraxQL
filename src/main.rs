use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use std::vec;

use hyraxql::commands::row_commands::GetRows;
use hyraxql::commands::table_commands::{ListHeaders, ListTables, SelTable};
use hyraxql::connection::factory::ConnectParams;
use hyraxql::engine::Engine;

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
    let mut url = url.trim().to_string();

    let mut tentativa = 1;

    let (_tx, _rx) = mpsc::channel();
    let mut engine = Engine::new(_tx);

    url = "postgres://myuser:mypassword@localhost:5432/mydatabase".into();

    loop {
        match engine.connect(ConnectParams::Url(url.clone())).await {
            Ok(()) => {
                println!("Connection established {} trys(s)!", tentativa);
                break;
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
    }

    println!("Connected!");

    if let Err(e) = engine.execute(ListTables).await {
        eprintln!("Error listing relations: {e}");
        return;
    }
    print!("{:#?}", engine.state().current_data);

    if let Err(e) = engine
        .execute(SelTable {
            nome: "people".into(),
        })
        .await
    {
        eprintln!("Error Setting relations: {e}");
        return;
    }

    if let Err(e) = engine.execute(ListHeaders).await {
        eprintln!("Error listing relations: {e}");
        return;
    }
    print!("{:#?}", engine.state().current_data);

    if let Err(e) = engine
        .execute(GetRows {
            size: 10,
            cols: vec!["id".to_owned(), "fname".to_owned()],
        })
        .await
    {
        eprintln!("Error listing relations: {e}");
        return;
    }

    print!("{:#?}", engine.state().current_data);
}
