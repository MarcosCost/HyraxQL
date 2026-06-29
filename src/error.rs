use std::fmt;

/// Unified error type for the HyraxQL engine.
///
/// Every layer of the application (connection, commands, engine)
/// produces this error type, which keeps error handling uniform
/// and makes it easy for UI consumers to display failures.
#[derive(Debug)]
pub enum HyraxError {
    /// Something went wrong while establishing or managing a connection.
    ConnectionError(String),
    /// A query against the data source failed.
    QueryError(String),
    /// The configuration provided for a connection is invalid.
    InvalidConfig(String),
    /// An internal engine error occurred (e.g., no connection active).
    EngineError(String),
    /// The event channel to the UI was closed.
    ChannelClosed,
}

impl fmt::Display for HyraxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HyraxError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            HyraxError::QueryError(msg) => write!(f, "Query error: {}", msg),
            HyraxError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            HyraxError::EngineError(msg) => write!(f, "Engine error: {}", msg),
            HyraxError::ChannelClosed => write!(f, "Channel closed"),
        }
    }
}

impl std::error::Error for HyraxError {}

impl From<std::sync::mpsc::SendError<u16>> for HyraxError {
    fn from(_: std::sync::mpsc::SendError<u16>) -> Self {
        HyraxError::ChannelClosed
    }
}

impl From<String> for HyraxError {
    fn from(msg: String) -> Self {
        HyraxError::EngineError(msg)
    }
}

impl From<&str> for HyraxError {
    fn from(msg: &str) -> Self {
        HyraxError::EngineError(msg.to_owned())
    }
}
