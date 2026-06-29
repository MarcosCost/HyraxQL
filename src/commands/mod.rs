use async_trait::async_trait;

use crate::connection::Connection;
use crate::engine::state::AppState;
use crate::error::HyraxError;

pub type Result<T> = std::result::Result<T, HyraxError>;

/// A single unit of work that the engine can execute against the
/// current connection.
///
/// Commands are the **only** way to interact with data sources.
/// They receive a reference to the active `Connection` and a
/// mutable reference to `AppState` to store results.  Because
/// commands depend only on the `Connection` trait, they work
/// unchanged across all backends.
///
/// # Example
///
/// ```ignore
/// pub struct ListTables;
///
/// #[async_trait]
/// impl Command for ListTables {
///     async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()> {
///         let tables = conn.list_relations().await?;
///         state.set(ManagerData::Tables(tables));
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Command: Send {
    /// Execute this command.
    ///
    /// `conn` is guaranteed to be a live connection when the engine
    /// calls this method.
    async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()>;
}

pub mod list_tables;
