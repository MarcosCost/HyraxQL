use std::sync::mpsc::Sender;

/// The data payload that the engine sends to the UI layer.
///
/// Each variant represents a distinct visual state the UI can render.
/// This is intentionally kept simple (all values as strings where
/// possible) so that GUI and TUI consumers don't need to perform
/// complex type conversions.
#[derive(Debug, Clone)]
pub enum ManagerData {
    Tables(Vec<String>),
    Rows(Vec<Vec<String>>),
    Columns(Vec<(String, String)>),
    ScalarInt(i64),
    ScalarString(String),
    CommandError(String),
    Loading,
    Idle,
}

#[derive(Debug)]
pub struct AppState {
    pub current_data: ManagerData,
    pub select_table: Option<String>,
    event_tx: Sender<u16>,
}

impl AppState {
    pub fn new(event_tx: Sender<u16>) -> Self {
        Self {
            current_data: ManagerData::Idle,
            select_table: None,
            event_tx,
        }
    }

    pub fn current_data(&self) -> &ManagerData {
        &self.current_data
    }

    /// Replace the current data and broadcast a refresh signal.
    pub fn set(&mut self, data: ManagerData) {
        self.current_data = data;
        let _ = self.event_tx.send(1);
    }
}
