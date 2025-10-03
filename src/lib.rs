pub mod constants;
pub mod database;
pub mod game_logic;
pub mod resources;
pub mod ui;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use qrcode::{Color as QrColor, QrCode};

use database::Database;
use game_logic::{load_leaderboard, load_user_data};
use resources::*;
use ui::ui_system;

fn configure_egui_fonts(mut contexts: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();
    let font_data = egui::FontData::from_static(include_bytes!("../assets/fonts/Poppins-Regular.ttf"));
    fonts.font_data.insert("poppins".to_owned(), font_data);

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "poppins".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "poppins".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}

fn generate_qr_codes(
    mut qr_textures: ResMut<QRCodeTextures>,
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    let android_url = "https://github.com/mintykiera/snake-game/releases/download/v1.0/snake_game.apk";
    let code = QrCode::new(android_url.as_bytes()).unwrap();
    let width = code.width();
    let size = [width, width];

    let pixels: Vec<egui::Color32> = code
        .to_colors()
        .into_iter()
        .map(|color| match color {
            QrColor::Dark => egui::Color32::BLACK,
            QrColor::Light => egui::Color32::WHITE,
        })
        .collect();

    let egui_image = egui::ColorImage { size, pixels };
    let texture_handle = ctx.load_texture("android_qr", egui_image, Default::default());
    qr_textures.android_qr = Some(texture_handle.id());

    let ios_url = "https://github.com/mintykiera/snake-game/releases/download/v1.0/snake_game.ipa";
    let code = QrCode::new(ios_url.as_bytes()).unwrap();
    let width = code.width();
    let size = [width, width];
    
    let pixels: Vec<egui::Color32> = code
        .to_colors()
        .into_iter()
        .map(|color| match color {
            QrColor::Dark => egui::Color32::BLACK,
            QrColor::Light => egui::Color32::WHITE,
        })
        .collect();
    
    let egui_image = egui::ColorImage { size, pixels };
    let texture_handle = ctx.load_texture("ios_qr", egui_image, Default::default());
    qr_textures.ios_qr = Some(texture_handle.id());
}

pub fn main() {
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
            }),
        )
        .insert_resource(WinitSettings::game())
        .add_plugins(EguiPlugin)
        .init_resource::<GameState>()
        .init_resource::<Game>()
        .init_resource::<UserProfile>()
        .init_resource::<Leaderboard>()
        .init_resource::<EguiInitialized>()
        .init_resource::<QRCodeTextures>()
        .init_resource::<Database>()
        .add_systems(Startup, (setup, load_user_data, load_leaderboard, configure_egui_fonts, generate_qr_codes))
        .add_systems(Update, ui_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}