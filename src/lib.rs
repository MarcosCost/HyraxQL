//! # HyraxQL Engine
//!
//! The library is split into four layers:
//!
//! - connection — The Connection trait and its implementations.
//!   Add new backends by implementing Connection and registering the
//!   variant in ConnectionConfig / ConnectionFactory.
//!
//! - commands — The Command trait and concrete commands.
//!   Commands receive a &dyn Connection and never depend on concrete
//!   backends.
//!
//! - engine — `Engine` orchestrates the connection and state.
//!   It is the main public API that UI consumers interact with.
//!
//! - error — Unified HyraxError type used everywhere.

pub mod commands;
pub mod connection;
pub mod engine;
pub mod error;
