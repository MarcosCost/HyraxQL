//  List of commands defined here:
// list_headers - lists the headers of the selected table
// list_tables - lists all available tables
// select_table - selects a table by name
//
use async_trait::async_trait;

use crate::commands::{Command, Result};
use crate::connection::Connection;
use crate::engine::state::{AppState, ManagerData};
/*--------------------------------------------------------------------------------------------------------------------------------------- */
pub struct ListHeaders;

#[async_trait]
impl Command for ListHeaders {
    async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()> {
        let names = conn
            .list_rel_headers(state.select_table.clone().unwrap().as_str())
            .await?;
        state.set(ManagerData::Columns(names));
        Ok(())
    }
}
/*--------------------------------------------------------------------------------------------------------------------------------------- */
pub struct ListTables;

#[async_trait]
impl Command for ListTables {
    async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()> {
        let names = conn.list_relations().await?;
        state.set(ManagerData::Tables(names));
        Ok(())
    }
}
/*--------------------------------------------------------------------------------------------------------------------------------------- */
pub struct SelTable {
    pub nome: String,
}

#[async_trait]
impl Command for SelTable {
    async fn execute(&self, _conn: &dyn Connection, state: &mut AppState) -> Result<()> {
        state.select_table = self.nome.clone().into();
        Ok(())
    }
}
/*--------------------------------------------------------------------------------------------------------------------------------------- */
