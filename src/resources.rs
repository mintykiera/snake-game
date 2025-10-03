use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use crate::constants::*;
use bevy_egui::egui; 

#[derive(Resource, Default, PartialEq, Clone)]
pub enum Screen {
    #[default]
    MainMenu,
    Playing,
    Settings,
    Leaderboard,
    Profile,
    Share,
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub current_screen: Screen,
}

#[derive(Resource)]
pub struct Game {
    pub snake: Vec<(i32, i32)>,
    pub direction: Direction,
    pub food: (i32, i32),
    pub score: u32,
    pub game_over: bool,
    pub paused: bool,
    pub timer: f32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            snake: vec![(10, 10), (10, 11), (10, 12)],
            direction: Direction::Up,
            food: (5, 5),
            score: 0,
            game_over: false,
            paused: false,
            timer: 0.0,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub snake_color: [u8; 3],
    pub background_color: [u8; 3],
    pub apple_color: [u8; 3],
    pub high_score: u32,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            user_id: generate_random_id(),
            username: String::new(),
            snake_color: DEFAULT_SNAKE_COLOR,
            background_color: DEFAULT_BACKGROUND_COLOR,
            apple_color: DEFAULT_APPLE_COLOR,
            high_score: 0,
        }
    }
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub username: String,
    pub score: u32,
}

#[derive(Resource, Default)]
pub struct EguiInitialized {
    pub initialized: bool,
}

#[derive(Resource, Default)]
pub struct QRCodeTextures {
    pub android_qr: Option<egui::TextureId>,
    pub ios_qr: Option<egui::TextureId>,
}

pub fn generate_random_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}