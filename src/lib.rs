//! # HyraxQL Engine
//!
//! A lightweight, extensible database-exploration engine that serves
//! as the backend for both GUI and TUI applications.
//!
//! ## Architecture
//!
//! The library is split into four layers:
//!
//! - **`connection`** — The `Connection` trait and its implementations.
//!   Add new backends by implementing `Connection` and registering the
//!   variant in `ConnectionConfig` / `ConnectionFactory`.
//!
//! - **`commands`** — The `Command` trait and concrete commands.
//!   Commands receive a `&dyn Connection` and never depend on concrete
//!   backends.
//!
//! - **`engine`** — `Engine` orchestrates the connection and state.
//!   It is the main public API that UI consumers interact with.
//!
//! - **`error`** — Unified `HyraxError` type used everywhere.
//!
//! ## Quick start
//!
//! ```ignore
//! use hyraxql::engine::Engine;
//! use hyraxql::commands::Command;
//! use hyraxql::commands::list_tables::ListTables;
//! use hyraxql::connection::ConnectionConfig;
//! use std::collections::HashMap;
//!
//! let (tx, _rx) = std::sync::mpsc::channel();
//! let mut engine = Engine::new(tx);
//!
//! let config = ConnectionConfig::Sqlite {
//!     path: ":memory:".into(),
//!     extra_params: HashMap::new(),
//! };
//!
//! tokio_test::block_on(async {
//!     engine.connect(config).await.unwrap();
//!     engine.execute(ListTables).await.unwrap();
//!     println!("{:#?}", engine.state().current_data());
//! });
//! ```

pub mod commands;
pub mod connection;
pub mod engine;
pub mod error;
