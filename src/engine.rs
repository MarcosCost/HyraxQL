use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;
use std::path;
use std::sync::mpsc::Sender;

use keyring::{Entry, Error};
use users::{get_current_uid, get_user_by_uid};

use crate::commands::Command;
use crate::connection::ConnectionFactory;
use crate::connection::Result as ConnResult;
use crate::connection::factory::ConnectParams;
use crate::engine::state::AppState;
use crate::engine::state::ManagerData;
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

    /// Save the Current connection to bookmarks (~/.local/hyraxql/bookmarks)
    // Needs to be called at the same time as connect cause sqlx doesnt hold the url anywhere
    pub fn save_profile(&mut self, url: &str) -> Result<(), HyraxError> {
        let user: String = get_user_by_uid(get_current_uid())
            .unwrap()
            .name()
            .to_str()
            .unwrap()
            .to_owned();

        // Check if keyring service exists
        if !keyring_backend_available() {
            self.state.current_data = ManagerData::ScalarString("No Keyring service available, Data will be stored unencrypted in .local/hyraxql/bookmarks/_user_".to_owned());
            write_to_local(&format!("bookmarks/{}", user), name, content);
            return Ok(());
        }
        //Check if hyraxql encryption key exists

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

/// Checks wheter or not a keyring service is available in the system
fn keyring_backend_available() -> bool {
    let entry = match Entry::new("keyring-probe", "probe-user") {
        Ok(e) => e,
        Err(_) => return false,
    };

    match entry.get_password() {
        Ok(_) => true,
        Err(Error::NoEntry) => true,
        Err(Error::NoStorageAccess(_)) => false,
        Err(Error::PlatformFailure(_)) => false,
        Err(_) => false,
    }
}

/// Writes to .local/hyrax/`path`
fn write_to_local(path_from_hyrax: &str, file_name: Option<&str>, content: &str) -> bool {
    let home = match dirs::home_dir() {
        Some(home) => home,
        None => return false,
    };

    let dir_path = home.join(".local/hyrax").join(path_from_hyrax);
    if create_dir_all(&dir_path).is_err() {
        return false;
    }

    let name;
    match file_name {
        Some(newname) => name = newname,
        None => name = content,
    }

    let file_path = dir_path.join(name);
    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(_) => return false,
    };

    file.write_all(content.as_bytes()).is_ok()
}
