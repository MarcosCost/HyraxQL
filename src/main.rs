use std::collections::HashMap;
use std::sync::mpsc::channel;

use hyraxql::commands::list_tables::ListTables;
use hyraxql::connection::ConnectionConfig;
use hyraxql::engine::Engine;

#[tokio::main]
async fn main() {
    let (tx, _rx) = channel();
    let mut engine = Engine::new(tx);

    let config = ConnectionConfig::Postgres {
        host: "localhost".to_owned(),
        port: 5432,
        user: "myuser".to_owned(),
        password: "mypassword".to_owned(),
        database: "mydatabase".to_owned(),
        extra_params: HashMap::new(),
    };

    engine.connect(config).await.unwrap();

    engine.execute(ListTables).await.unwrap();

    println!("{:#?}", engine.state().current_data());
}
