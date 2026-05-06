//! 数据库模块
//!
//! 提供 SQLite 数据库连接池管理和数据持久化

pub mod connection;
pub mod schema;
pub mod models;
pub mod commands;
pub mod backup;

pub use connection::init_database;
