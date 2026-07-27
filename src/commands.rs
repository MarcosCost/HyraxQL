use async_trait::async_trait;

use crate::connection::Connection;
use crate::engine::state::AppState;
use crate::error::HyraxError;

pub type Result<T> = std::result::Result<T, HyraxError>;

#[async_trait]
pub trait Command: Send {
    async fn execute(&self, conn: &dyn Connection, state: &mut AppState) -> Result<()>;
}

pub mod row_commands;
pub mod table_commands;
