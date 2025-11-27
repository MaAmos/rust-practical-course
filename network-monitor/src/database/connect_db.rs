use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
use std::sync::Arc;
use crate::tools::retry_tool::default_retry_policy;
use backon::Retryable;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!(); // 默认 查找与cargo.toml同级目录下的migrations文件夹
pub fn establish_connection() ->Result<SqlitePool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "./db/monitor.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool  = Pool::builder()
        .max_size(8)
        .connection_timeout(std::time::Duration::from_secs(30))
        .build(manager)
        .map_err(|e| format!("Failed to create DB pool: {}", e))?;
    let mut conn =   pool.get().map_err(|e| format!("Failed to get DB connection from pool: {}", e))?;
    conn.run_pending_migrations(MIGRATIONS).map_err(|e| format!("Failed to run database migrations: {}", e))?;
    Ok(pool)
}

pub async fn establish_database_connection() -> Result<Arc<SqlitePool>, Box<dyn std::error::Error>> {
    // 引入 backon 库 来实现链接方法重试的逻辑
    let pool = (|| async { establish_connection() })
        .retry(default_retry_policy())
        .await
        .map(Arc::new)
        .map_err(|e| {
            eprintln!("数据库连接重试失败: {}", e);
            e
        })?;
    Ok(pool)
}

pub fn get_connection(pool: &SqlitePool) -> SqlitePooledConnection {
    pool.get().expect("Failed to get connection from pool.")
}
