use bevy::prelude::Resource;
use sled::Db;

#[derive(Resource)]
pub struct Database {
    pub db: Db,
}

impl Default for Database {
    fn default() -> Self {
        let db = sled::open("snake_game_data.db").expect("Failed to open database");
        Self { db }
    }
}