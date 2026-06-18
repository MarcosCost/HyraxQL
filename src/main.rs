use std::sync::mpsc::channel;
use crate::{app_state::{AppState, ManagerData::ScalarInt}, commands::conn_init, misc::app_structs};

use crate::commands::conn_init::connect;

mod app_state;
pub mod commands {
    pub mod conn_init;
}
pub mod db {
    pub mod sqlx_impl;
    pub mod database;
}
pub mod misc {
    pub mod app_enums;
    pub mod app_structs;
}

#[tokio::main]
async fn main() { 
    let (tx, rx) = channel();       // Create the comunication channel for the engine to publidh events to the UI
    let mut state = AppState::new(tx);
    
    let conn_args = app_structs::ConnectionArgs {
        db_type: "postgres".to_owned(),
        db_name: "mydatabase".to_owned(),
        db_user: "myuser".to_owned(),
        db_pass: "mypassword".to_owned(),
        host: "localhost".to_owned(),
        port: 5432,
        extra_params: None
    };

    connect(&mut state, conn_args).await;

}