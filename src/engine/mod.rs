use std::sync::mpsc::Sender;

use crate::commands::Command;
use crate::connection::Result as ConnResult;
use crate::connection::{ConnectionConfig, ConnectionFactory};
use crate::engine::state::AppState;
use crate::error::HyraxError;

pub mod state;

/// The central orchestrator of the HyraxQL engine.
///
/// The `Engine` owns the active connection and the application state.
/// Consumers (GUI / TUI) interact with it by:
///
/// 1. Creating an `Engine` with an event channel sender.
/// 2. Calling `connect()` to establish a connection.
/// 3. Calling `execute()` with a `Command` to perform work.
/// 4. Reading `state()` to update the UI.
///
/// # Extensibility
///
/// Because `Engine` talks to connections through the `Connection`
/// trait and to operations through the `Command` trait, both axes
/// can be extended without modifying the engine itself.
pub struct Engine {
    connection: Option<Box<dyn crate::connection::Connection>>,
    state: AppState,
}

impl Engine {
    /// Create a new engine.
    ///
    /// `event_tx` is the channel used to signal the UI that state
    /// has changed (the engine sends `1` on every `set` call).
    pub fn new(event_tx: Sender<u16>) -> Self {
        Self {
            connection: None,
            state: AppState::new(event_tx),
        }
    }

    /// Establish a connection using the given configuration.
    ///
    /// This replaces any previously active connection.
    pub async fn connect(&mut self, config: ConnectionConfig) -> ConnResult<()> {
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
