// error.rs
use std::fmt;

pub type HyraxResult<T> = Result<T, HyraxError>;

#[derive(Debug)]
pub enum HyraxError {
    // --- Engine lifecycle ---
    EngineInit(String),
    ConnectFailed {
        driver_id: String,
        reason: String,
    },
    NotConnected,
    DisconnectFailed(String),

    // --- Queries ---
    QueryFailed(String),
    InvalidIntent(String),
    TranslationFailed(String),
    QueryTimeout(u64),

    // --- Drivers ---
    DriverNotFound(String),
    MissingConnectionField(String),
    InvalidConnectionField {
        field: String,
        reason: String,
    },

    // --- Schema ---
    NamespaceNotFound(String),
    EntityNotFound(String),
    SchemaIntrospectionFailed(String),

    // --- Plugins ---
    PluginNotFound(String),
    PluginAlreadyInstalled(String),
    DependencyResolutionFailed {
        plugin: String,
        reason: String,
    },
    InvalidManifest {
        path: String,
        reason: String,
    },
    WasmLoadFailed(String, String),
    WasmCallFailed {
        plugin: String,
        func: String,
        reason: String,
    },
    WasmFileNotFound(String),

    // --- Settings / Bookmarks / config ---
    BookmarkNotFound(u64),
    ConfigDirNotFound,

    // --- Passthrough / generic ---
    Io(std::io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    Other(String),
}

impl fmt::Display for HyraxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EngineInit(msg) => write!(f, "engine failed to initialize: {msg}"),
            Self::ConnectFailed { driver_id, reason } => {
                write!(f, "failed to connect using driver '{driver_id}': {reason}")
            }
            Self::NotConnected => write!(f, "no active connection"),
            Self::DisconnectFailed(msg) => write!(f, "failed to disconnect: {msg}"),

            Self::QueryFailed(msg) => write!(f, "query failed: {msg}"),
            Self::InvalidIntent(msg) => write!(f, "invalid query intent: {msg}"),
            Self::TranslationFailed(msg) => {
                write!(f, "could not translate intent to driver query: {msg}")
            }
            Self::QueryTimeout(ms) => write!(f, "query timed out after {ms}ms"),

            Self::DriverNotFound(id) => write!(f, "driver '{id}' not found"),
            Self::MissingConnectionField(field) => {
                write!(f, "missing required connection field '{field}'")
            }
            Self::InvalidConnectionField { field, reason } => {
                write!(f, "invalid connection field '{field}': {reason}")
            }

            Self::NamespaceNotFound(ns) => write!(f, "namespace '{ns}' not found"),
            Self::EntityNotFound(e) => write!(f, "entity '{e}' not found"),
            Self::SchemaIntrospectionFailed(msg) => {
                write!(f, "failed to introspect schema: {msg}")
            }

            Self::PluginNotFound(id) => write!(f, "plugin '{id}' not found"),
            Self::PluginAlreadyInstalled(id) => {
                write!(f, "plugin '{id}' is already installed")
            }
            Self::DependencyResolutionFailed { plugin, reason } => {
                write!(
                    f,
                    "failed to resolve dependencies for plugin '{plugin}': {reason}"
                )
            }
            Self::InvalidManifest { path, reason } => {
                write!(f, "invalid plugin manifest at {path}: {reason}")
            }
            Self::WasmLoadFailed(id, reason) => {
                write!(f, "failed to load wasm module '{id}': {reason}")
            }
            Self::WasmCallFailed {
                plugin,
                func,
                reason,
            } => {
                write!(
                    f,
                    "failed to call wasm function '{func}' in plugin '{plugin}': {reason}"
                )
            }
            Self::WasmFileNotFound(id) => write!(f, "no .wasm file found for plugin '{id}'"),

            Self::BookmarkNotFound(id) => write!(f, "bookmark '{id}' not found"),
            Self::ConfigDirNotFound => write!(f, "could not find config folder"),

            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::TomlDe(e) => write!(f, "TOML parse error: {e}"),
            Self::TomlSer(e) => write!(f, "TOML serialize error: {e}"),
            Self::Other(msg) => write!(f, "unexpected error: {msg}"),
        }
    }
}

impl std::error::Error for HyraxError {}

impl From<std::io::Error> for HyraxError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::de::Error> for HyraxError {
    fn from(e: toml::de::Error) -> Self {
        Self::TomlDe(e)
    }
}

impl From<toml::ser::Error> for HyraxError {
    fn from(e: toml::ser::Error) -> Self {
        Self::TomlSer(e)
    }
}
