#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use snake_game::SnakeApp;

#[cfg(not(target_os = "android"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    let _ = dotenvy::dotenv();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 800.0])
            .with_min_inner_size([300.0, 600.0])
            .with_title("Snake Game"),
        ..Default::default()
    };

    eframe::run_native(
        "Snake Game",
        native_options,
        Box::new(|cc| Ok(Box::new(SnakeApp::new(cc)))),
    )
}

#[cfg(target_os = "android")]
fn main() {
}