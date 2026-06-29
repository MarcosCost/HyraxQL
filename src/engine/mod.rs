use std::sync::mpsc::Sender;

use crate::commands::Command;
use crate::connection::ConnectionFactory;
use crate::connection::Result as ConnResult;
use crate::connection::factory::ConnectParams;
use crate::engine::state::AppState;
use crate::error::HyraxError;

pub mod state;

pub struct Engine {
    connection: Option<Box<dyn crate::connection::Connection>>,
    state: AppState,
}

impl Engine {
    /// `event_tx` is the channel used to signal the UI that state
    /// has changed (the engine sends `1` on every `set` call).
    pub fn new(event_tx: Sender<u16>) -> Self {
        Self {
            connection: None,
            state: AppState::new(event_tx),
        }
    }

    pub async fn connect(&mut self, config: ConnectParams) -> ConnResult<()> {
        let conn = ConnectionFactory::connect(config).await?;
        self.connection = Some(conn);
        Ok(())
    }

    /// Run a command against the active connection.
    ///
    /// Returns an error if no connection is active.
    pub async fn execute(&mut self, command: impl Command) -> Result<(), HyraxError> {
        let conn = self
            .connection
            .as_ref()
            .ok_or_else(|| HyraxError::EngineError("No active connection".into()))?;

        command.execute(conn.as_ref(), &mut self.state).await?;
        Ok(())
    }

    /// Immutable reference to the application state.
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Returns `true` when a connection is active.
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }
}
