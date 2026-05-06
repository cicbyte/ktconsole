//! 数据库连接池管理
//!
//! 使用 r2d2 提供 SQLite 连接池

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use super::schema;

/// 数据库连接池类型
pub type DbPool = Pool<SqliteConnectionManager>;

/// 数据库连接类型
pub type DbConnection = PooledConnection<SqliteConnectionManager>;

/// 全局数据库连接池
static DB_POOL: OnceLock<Mutex<Option<DbPool>>> = OnceLock::new();

/// 初始化数据库
///
/// 创建数据库文件和表结构
pub fn init_database(data_dir: &PathBuf) -> Result<(), String> {
    // 确保数据目录存在
    if let Err(e) = std::fs::create_dir_all(data_dir) {
        return Err(format!("创建数据目录失败: {}", e));
    }

    // 数据库文件路径
    let db_path = data_dir.join("tauri_template.db");

    // 创建连接管理器
    let manager = SqliteConnectionManager::file(&db_path);

    // 创建连接池
    let pool = Pool::new(manager)
        .map_err(|e| format!("创建数据库连接池失败: {}", e))?;

    // 初始化表结构
    let conn = pool.get()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    schema::init_schema(&conn)?;

    // 保存连接池
    let mut db_pool = DB_POOL
        .get_or_init(|| Mutex::new(None))
        .lock()
        .map_err(|e| format!("锁定连接池失败: {}", e))?;
    *db_pool = Some(pool);

    println!("数据库初始化完成: {:?}", db_path);
    Ok(())
}

/// 获取数据库连接池
pub fn get_pool() -> Result<DbPool, String> {
    let pool_guard = DB_POOL
        .get_or_init(|| Mutex::new(None))
        .lock()
        .map_err(|e| format!("锁定连接池失败: {}", e))?;

    pool_guard.clone().ok_or_else(|| "数据库未初始化".to_string())
}

/// 获取数据库连接
pub fn get_connection() -> Result<DbConnection, String> {
    let pool = get_pool()?;
    pool.get().map_err(|e| format!("获取数据库连接失败: {}", e))
}
