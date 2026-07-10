use std::ops::Deref;
use std::sync::mpsc::Sender;

use keyring::Entry;
use users::{get_current_uid, get_user_by_uid};

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

    // TODO: checkif there is a keyring service active and warn about unencrypted data if so. cause I dont lol
    /// Save the Current connection to bookmarks (~/.local/hyraxql/bookmarks)
    pub fn save_profile(&self) -> Result<(), HyraxError> {
        //get user logged as a String
        let user: String = get_user_by_uid(get_current_uid())
            .unwrap()
            .name()
            .to_str()
            .unwrap()
            .to_owned();
        // Check if eyring service exists

        //Check if hyraxql encryption key exists
        let key128: u128;
        match Entry::get_password(
            &(Entry::new("hyraxql", format!("{}_hyraxql", user).deref()).unwrap()),
        ) {
            Ok(key) => key128 = key.parse::<u128>().unwrap(),
            Err(_e) => key128 = rand::random(),
        }
        println!("{}", key128);
        // if not create else create
        // take url encript over aes-csm key hyraxql
        // print EncUrl to ~/.local/hyrax/user_connects

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
