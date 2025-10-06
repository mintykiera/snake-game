use bevy::prelude::Resource;
use std::collections::HashMap;
use std::fs;

#[derive(Resource)]
pub struct Database {
    data: HashMap<String, String>,
    data_dir: String,
}

impl Database {
    pub fn new(data_dir: String) -> Self {
        fs::create_dir_all(&data_dir).ok();
        Self {
            data: HashMap::new(),
            data_dir,
        }
    }

    pub fn save_data(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        let file_path = format!("{}/{}.json", &self.data_dir, key);
        if let Err(e) = fs::write(file_path, value) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    pub fn load_data(&self, key: &str) -> Option<String> {
        if let Some(value) = self.data.get(key) {
            return Some(value.clone());
        }
        let file_path = format!("{}/{}.json", &self.data_dir, key);
        fs::read_to_string(file_path).ok()
    }
}