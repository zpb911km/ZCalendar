use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub db_url: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_url: None,
        }
    }
}

pub struct ConfigManager {
    config_path: std::path::PathBuf,
    config: Mutex<AppConfig>,
}

impl ConfigManager {
    pub fn new<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<Self, Box<dyn std::error::Error>> {
        // 使用 Tauri 的 path API 获取配置目录
        let config_dir = app_handle.path().app_config_dir()?;
        
        // 确保配置目录存在
        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.json");
        
        // 初始化配置
        let config = Self::load_from_path(&config_path).unwrap_or_default();

        Ok(Self {
            config_path,
            config: Mutex::new(config),
        })
    }

    fn load_from_path(path: &std::path::Path) -> Option<AppConfig> {
        if !path.exists() {
            return None;
        }

        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).ok(),
            Err(_) => None,
        }
    }

    pub fn load_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn save_config(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        
        // 更新内存中的配置
        *self.config.lock().unwrap() = config.clone();
        
        Ok(())
    }
}