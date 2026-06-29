use async_trait::async_trait;

use crate::commands::{Command, Result};
use crate::connection::Connection;
use crate::engine::state::{AppState, ManagerData};

/// Lists all tables / collections / relations in the connected data
/// source and stores the result in `ManagerData::Tables`.
pub struct ListTables;

#[async_trait]
impl Command for ListTables {
    async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()> {
        let names = conn.list_relations().await?;
        state.set(ManagerData::Tables(names));
        Ok(())
    }
}
