use bevy::prelude::Resource;
use std::collections::HashMap;
use std::fs;

#[derive(Resource)]
pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self { 
            data: HashMap::new(),
        }
    }

    pub fn save_data(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        
        let data_dir = if cfg!(target_os = "android") || cfg!(target_os = "ios") {
            std::env::var("APP_DATA_DIR")
                .unwrap_or_else(|_| ".".to_string())
        } else {
            directories::BaseDirs::new()
                .unwrap()
                .data_dir()
                .join("snake_game")
                .to_string_lossy()
                .to_string()
        };
        
        std::fs::create_dir_all(&data_dir).ok();
        let file_path = format!("{}/{}.json", data_dir, key);
        if let Err(e) = fs::write(file_path, value) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    pub fn load_data(&self, key: &str) -> Option<String> {
        if let Some(value) = self.data.get(key) {
            return Some(value.clone());
        }
        
        let data_dir = if cfg!(target_os = "android") || cfg!(target_os = "ios") {
            std::env::var("APP_DATA_DIR").unwrap_or_else(|_| ".".to_string())
        } else {
            directories::BaseDirs::new()
                .unwrap()
                .data_dir()
                .join("snake_game")
                .to_string_lossy()
                .to_string()
        };
        
        let file_path = format!("{}/{}.json", data_dir, key);
        fs::read_to_string(file_path).ok()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}