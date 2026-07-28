use std::{
    fs::{File, create_dir_all},
    io::Write,
    sync::mpsc::Sender,
    time::{SystemTime, UNIX_EPOCH},
};

use aes_gcm::{
    Aes256Gcm, Key, KeyInit, Nonce,
    aead::{Aead, Generate},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use keyring::{Entry, Error};
use users::{get_current_uid, get_user_by_uid};

use crate::{
    commands::Command,
    connection::{ConnectionFactory, Result as ConnResult, factory::ConnectParams},
    engine::state::{AppState, ManagerData},
    error::HyraxError,
};

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

    /// Save the Current connection to bookmarks (~/.local/hyraxql/bookmarks/user)
    pub fn save_profile(&mut self, url: &str) -> Result<(), HyraxError> {
        let user = get_user_by_uid(get_current_uid())
            .ok_or_else(|| HyraxError::EngineError("User not found".to_owned()))?
            .name()
            .to_str()
            .ok_or_else(|| HyraxError::EngineError("Invalid username".to_owned()))?
            .to_owned();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        let dir_path = format!("bookmarks/{}", user);
        let file_name = format!("Connection_{}", timestamp);

        // Check if keyring service exists and save it unencrypted if not
        if !keyring_backend_available() {
            self.state.current_data = ManagerData::ScalarString(
                "No Keyring service available, Data will be stored unencrypted in .local/hyraxql/bookmarks/_user_".to_owned()
            );
            write_to_local(&dir_path, Some(&file_name), url);
            return Ok(());
        }

        // Get/create an encryption key for this user
        let entry = Entry::new("hyraxql", &user)
            .map_err(|e| HyraxError::EngineError(format!("Failed to access keyring: {e}")))?;

        let key_bytes: Vec<u8> = match entry.get_secret() {
            Ok(bytes) => bytes,
            Err(Error::NoEntry) => {
                let local_key = Key::<Aes256Gcm>::generate();
                entry
                    .set_secret(local_key.as_slice())
                    .map_err(|e| HyraxError::EngineError(format!("Failed to save new key: {e}")))?;
                local_key.to_vec()
            }
            Err(e) => {
                return Err(HyraxError::EngineError(format!(
                    "Unhandled Error getting/creating encrypt key: {e}"
                )));
            }
        };
        let key: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| HyraxError::EngineError("stored key has unexpected length".into()))?;

        // Encrypt URL over AES-GCM
        let cipher = Aes256Gcm::new(&key.into());
        let nonce = Nonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, url.as_bytes())
            .map_err(|e| HyraxError::EngineError(format!("Encryption failed: {e}")))?;

        let mut combined_payload = nonce.to_vec();
        combined_payload.extend_from_slice(&ciphertext);

        let encoded_data = BASE64.encode(&combined_payload);

        write_to_local(&dir_path, Some(&file_name), &encoded_data);

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

    let name = match file_name {
        Some(newname) => newname,
        None => content,
    };

    let file_path = dir_path.join(name);
    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(_) => return false,
    };

    file.write_all(content.as_bytes()).is_ok()
}
