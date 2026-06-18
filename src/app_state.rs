use std::sync::mpsc::Sender;
use crate::db::database::{DbPool};

pub enum ManagerData {
    Rows(Vec<Vec<String>>),
    Columns(Vec<(String,String)>),  // Vec of (Name, Type) tuples
    ScalarInt(i64),
    ScalarString(String),
    CommandError(String),
    Loading,
    Idle
}

pub struct AppState {
    pub db_pool: Option<Box<dyn DbPool>>,
    pub current_data: ManagerData,
    pub select_table: Option<String>,           // The currently selected table to Allow Omission in the UI's

    pub event_tx: Sender<u16>              // Send A signal to the reciever in the UI
}
impl AppState {
    // Instanciate a new AppState Object
    pub fn new(event_tx: Sender<u16>) -> Self {
        Self {
            current_data: ManagerData::Idle,
            event_tx,
            select_table: None,
            db_pool: None
        }
    }

    pub fn set(&mut self ,new_data: ManagerData){
        self.current_data = new_data;
        if let Err(e) = self.event_tx.send(1) {
            eprintln!("Failed to send refresh message: {}", e);
        }
    }
}