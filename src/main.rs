use std::sync::mpsc::channel;
use crate::app_state::{AppState, ManagerData::{ScalarInt}};

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

fn main() { 
    let (tx, rx) = channel();       // Create the comunication channel for the engine to publidh events to the UI
    let mut state = AppState::new(tx);
    
    

}