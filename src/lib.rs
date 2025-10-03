// In src/lib.rs

mod constants;
mod game_logic;
mod resources;
mod ui;
mod database;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use database::Database;
use resources::*;
use game_logic::*;
use ui::ui_system;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake Game".to_string(),
                    resolution: (400., 800.).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(EguiPlugin)
        .init_resource::<GameState>()
        .init_resource::<Game>()
        .init_resource::<UserProfile>()
        .init_resource::<Leaderboard>()
        .init_resource::<EguiInitialized>()
        .init_resource::<QRCodeTextures>()
        .init_resource::<Database>()
        .add_systems(Startup, (setup, load_user_data, load_leaderboard))
        .add_systems(Update, ui_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}