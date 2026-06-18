use std::sync::Mutex;

use async_trait::async_trait;
use sqlx::pool::PoolConnection;
use sqlx::{Any, AnyPool, Column, Row};
use crate::db::database::{DbPool, DbProvider};
use crate::app_state::ManagerData;
use crate::misc::app_enums::AppError;


// ==========================================
// Pool (DbPool)
// ==========================================

pub struct SqlxPoolImpl {
    pool: AnyPool,
}

impl SqlxPoolImpl {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DbPool for SqlxPoolImpl {
    async fn acquire(&self) -> Result<Box<dyn DbProvider>, AppError> {

        let conn = self.pool
            .acquire()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let wrapped_conn = SqlxConnection::new(conn);
        
        Ok(Box::new(wrapped_conn))
    }
}


// ==========================================
// Single Conn (DbProvider)
// ==========================================

pub struct SqlxConnection {
    conn: Mutex<PoolConnection<Any>>,
}

impl SqlxConnection {
    pub fn new(conn: PoolConnection<Any>) -> Self {
        Self { 
            conn: Mutex::new(conn) 
        }
    }
}

#[async_trait]
impl DbProvider for SqlxConnection {
    async fn connect(&self) -> Result<u32, AppError> {
        // SQLx connections fetched from a pool are already connected
        Ok(0)
    }

    async fn disconnect(&self) -> Result<u32, AppError> {
        // SQLx pool connections automatically return to the pool when they go out of scope (Drop).
        Ok(0)
    }
}