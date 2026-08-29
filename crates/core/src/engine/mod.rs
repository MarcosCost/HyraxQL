// engine/mod.rs

use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::bookmarks::Bookmarks;
use crate::drivers::{Driver, QueryResult};
use crate::error::*;
use crate::plugins::PluginRegistry;
use crate::settings::Settings;

/// A live connection to any database, regardless of type.
/// The actual driver (sqlx postgres, mongo, redis, etc.)
/// lives behind this trait object.
pub struct Connection {
    pub id: String,
    pub label: String,
    pub driver_id: String,
    pub driver: Arc<dyn Driver>,
}

/// The full application state owned by the engine.
/// TUI and GUI get an Arc<RwLock<AppState>> and talk to it.
pub struct AppState {
    /// All loaded plugins and drivers
    pub plugins: PluginRegistry,
    /// Active connections, keyed by connection id
    pub connections: HashMap<String, Connection>,
    /// Which connection is currently focused
    pub active_connection: Option<String>,
    /// User settings
    pub settings: Settings,
    /// Saved Connection Strings
    pub bookmarks: Bookmarks,
}

/// The engine is the single entry point for TUI/GUI.
/// It owns AppState and exposes high level operations.
pub struct Engine {
    state: Arc<RwLock<AppState>>,
}

impl Engine {
    /// Initializes the engine
    pub async fn new() -> HyraxResult<Self> {
        let path = dirs::config_dir();
        match path {
            Some(pathbuf) => {
                let hyraxql_dir = pathbuf.join("Hyraxql");
                let plugins_dir = hyraxql_dir.join("plugins");

                if fs::create_dir_all(&plugins_dir).is_ok() {
                    let _ = fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(hyraxql_dir.join("bookmarks.toml"));
                    let _ = fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(plugins_dir.join("manifest.toml"));
                }
            }
            None => {
                return Err(HyraxError::EngineInit(
                    "Couldn't find the config folder".to_owned(),
                ));
            }
        };

        Ok(Engine {
            state: Arc::new(RwLock::new(AppState {
                plugins: PluginRegistry::new().unwrap(),
                connections: HashMap::new(),
                active_connection: None,
                settings: Settings::new().unwrap(),
            })),
        })
    }

    /// Returns a clone of the current state so UI can acess something stable while the backend changes
    pub fn state(&self) -> Arc<RwLock<AppState>> {
        Arc::clone(&self.state)
    }

    /// Load a plugin from disk and register it
    pub async fn load_plugin(&self, path: &str) -> HyraxResult<()> {
        todo!()
    }

    /// Open a new connection using a registered driver
    pub async fn connect(&self, driver_id: &str, config: ConnectionConfig) -> HyraxResult<String> {
        todo!()
    }

    /// Execute a query on the active connection
    pub async fn execute(&self, query: &str) -> HyraxResult<QueryResult> {
        todo!()
    }

    /// Close a connection
    pub async fn disconnect(&self, connection_id: &str) -> HyraxResult<()> {
        todo!()
    }
}
