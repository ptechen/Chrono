pub mod avatars;
pub mod chat_info;
pub mod create_table;
pub mod friend;
pub mod owner_info;
pub mod verification_code;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use dirs;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub static SQLITE_POOL: Lazy<SqlitePool> = Lazy::new(|| {
    let config = SqlConfig::default();
    init_sqlite_db(config)
});

pub fn init_sqlite_db(config: SqlConfig) -> SqlitePool {
    SqlitePoolOptions::new().connect_lazy(&config.url).unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub max_lifetime: u64,
    pub idle_timeout: u64,
}

impl Default for SqlConfig {
    fn default() -> Self {
        let mut url = String::from(".chrono");
        let mut path = String::from(".chrono");
        let home = dirs::home_dir().unwrap_or_default();
        let home = home.to_str().unwrap_or_default();
        #[cfg(target_os = "macos")]
        {
            url = format!("{}/Library/Application Support/com.tauri.chrono/chrono.sqlite?mode=rwc", home);
            path = format!("{}/Library/Application Support/com.tauri.chrono", home);
        }
        #[cfg(target_os = "linux")]
        {
            url = format!("{}/.chrono/chrono.sqlite?mode=rwc", home);
        }

        #[cfg(target_os = "windows")]
        {
            url = format!("{}/.chrono/chrono.sqlite?mode=rwc", home);
        }

        let path = Path::new(&path);
        fs::create_dir_all(&path).unwrap();
        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).unwrap();
        println!("{url}");
        Self {
            url,
            max_connections: 64,
            min_connections: 1,
            acquire_timeout: 5,
            max_lifetime: 1800,
            idle_timeout: 600,
        }
    }
}
