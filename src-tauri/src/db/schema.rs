//! 数据库表结构定义

use rusqlite::Connection;

/// 初始化数据库表结构
pub fn init_schema(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS app_config (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            auto_reconnect INTEGER DEFAULT 0,
            log_level TEXT DEFAULT 'information',
            created_at INTEGER,
            updated_at INTEGER
        );

        CREATE TABLE IF NOT EXISTS provider_configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider_id TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            auth_token TEXT DEFAULT '',
            region TEXT DEFAULT '',
            extra_config TEXT DEFAULT '{}',
            created_at INTEGER,
            updated_at INTEGER
        );

        INSERT OR IGNORE INTO app_config (id, created_at, updated_at)
        VALUES (1, strftime('%s', 'now') * 1000, strftime('%s', 'now') * 1000);

        INSERT OR IGNORE INTO provider_configs (provider_id, name, created_at, updated_at)
        VALUES ('ngrok', 'ngrok', strftime('%s', 'now') * 1000, strftime('%s', 'now') * 1000);

        INSERT OR IGNORE INTO provider_configs (provider_id, name, created_at, updated_at)
        VALUES ('cpolar', 'cpolar', strftime('%s', 'now') * 1000, strftime('%s', 'now') * 1000);

        INSERT OR IGNORE INTO provider_configs (provider_id, name, created_at, updated_at)
        VALUES ('frp', 'FRP', strftime('%s', 'now') * 1000, strftime('%s', 'now') * 1000);

        INSERT OR IGNORE INTO provider_configs (provider_id, name, created_at, updated_at)
        VALUES ('gradio', 'Gradio', strftime('%s', 'now') * 1000, strftime('%s', 'now') * 1000);
        "#
    ).map_err(|e| format!("创建数据库表失败: {}", e))?;

    println!("数据库表结构初始化完成");

    // 版本迁移
    let current_version = get_db_version(conn)?;
    if current_version < 3 {
        // v2 -> v3: 增加 extra_config 列
        let _ = conn.execute(
            "ALTER TABLE provider_configs ADD COLUMN extra_config TEXT DEFAULT '{}'",
            []
        );
        conn.execute(
            "UPDATE db_version SET version = 3",
            []
        ).map_err(|e| format!("更新数据库版本失败: {}", e))?;
        println!("数据库已迁移到版本 3");
    }
    if current_version < 4 {
        // v3 -> v4: 新增 tunnels 表
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tunnels (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                provider TEXT NOT NULL,
                protocol TEXT NOT NULL,
                local_port TEXT NOT NULL,
                local_host TEXT NOT NULL DEFAULT 'localhost',
                custom_domain TEXT,
                basic_auth INTEGER DEFAULT 0,
                ip_whitelist INTEGER DEFAULT 0,
                compression INTEGER DEFAULT 0,
                inspect INTEGER DEFAULT 0,
                created_at INTEGER,
                updated_at INTEGER
            );
            "#
        ).map_err(|e| format!("创建 tunnels 表失败: {}", e))?;
        conn.execute(
            "UPDATE db_version SET version = 4",
            []
        ).map_err(|e| format!("更新数据库版本失败: {}", e))?;
        println!("数据库已迁移到版本 4");
    }
    if current_version < 5 {
        // v4 -> v5: tunnels 表添加 basic_auth_user / basic_auth_pass 列
        let _ = conn.execute(
            "ALTER TABLE tunnels ADD COLUMN basic_auth_user TEXT",
            []
        );
        let _ = conn.execute(
            "ALTER TABLE tunnels ADD COLUMN basic_auth_pass TEXT",
            []
        );
        conn.execute(
            "UPDATE db_version SET version = 5",
            []
        ).map_err(|e| format!("更新数据库版本失败: {}", e))?;
        println!("数据库已迁移到版本 5");
    }

    Ok(())
}

pub const DB_VERSION: i32 = 5;

pub fn get_db_version(conn: &Connection) -> Result<i32, String> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='db_version')",
        [],
        |row| row.get(0)
    ).map_err(|e| format!("检查版本表失败: {}", e))?;

    if !exists {
        conn.execute(
            "CREATE TABLE db_version (version INTEGER PRIMARY KEY)",
            []
        ).map_err(|e| format!("创建版本表失败: {}", e))?;

        conn.execute(
            "INSERT INTO db_version (version) VALUES (?)",
            [DB_VERSION]
        ).map_err(|e| format!("初始化版本失败: {}", e))?;

        return Ok(DB_VERSION);
    }

    conn.query_row("SELECT version FROM db_version", [], |row| row.get(0))
        .map_err(|e| format!("获取版本失败: {}", e))
}
