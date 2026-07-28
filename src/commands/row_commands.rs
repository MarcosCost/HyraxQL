// List of commands defined here:
// get_rows - gets the first x rows in a collumn, defaults 50, take collumn names to filter, returns all if no filter
//
use async_trait::async_trait;

use crate::commands::{Command, Result};
use crate::connection::Connection;
use crate::engine::state::{AppState, ManagerData};
/*--------------------------------------------------------------------------------------------------------------------------------------- */
pub struct GetRows {
    pub size: u32,
    pub page: u32,
    pub cols: Vec<String>,
}
impl Default for GetRows {
    fn default() -> GetRows {
        GetRows {
            size: 50,
            page: 1,
            cols: vec![],
        }
    }
}

#[async_trait]
impl Command for GetRows {
    async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()> {
        let rows = conn
            .get_rows(
                state.select_table.clone().unwrap().as_str(),
                self.size,
                self.page,
                self.cols.clone(),
            )
            .await?;
        state.set(ManagerData::Rows(rows));
        Ok(())
    }
}
/*--------------------------------------------------------------------------------------------------------------------------------------- */
