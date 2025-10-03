mod game_screen;
mod leaderboard;
mod main_menu;
mod profile;
mod settings;
mod share;

use game_screen::*;
use leaderboard::*;
use main_menu::*;
use profile::*;
use settings::*;
use share::*;

use bevy::prelude::*;
use bevy::input::Input;
use bevy_egui::{egui, EguiContexts};
use crate::resources::*;
use crate::constants::*;
use crate::game_logic::*;
use crate::database::Database;
use qrcode::QrCode;
use qrcode::render::unicode;

fn generate_qr_code(url: &str) -> Option<String> {
    if let Ok(code) = QrCode::new(url.as_bytes()) {
        let string = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Dark)
            .light_color(unicode::Dense1x2::Light)
            .build();
        return Some(string);
    }
    None
}

fn apply_modern_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
    
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(40);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(60);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 180, 0);
    
    ctx.set_style(style);
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<GameState>,
    mut game: ResMut<Game>,
    mut profile: ResMut<UserProfile>,
    mut leaderboard: ResMut<Leaderboard>,
    time: Res<Time>,
    mut egui_init: ResMut<EguiInitialized>,
    mut qr_textures: ResMut<QRCodeTextures>,
    keyboard: Res<Input<KeyCode>>,
    db_res: Res<Database>,
) {
    let ctx = contexts.ctx_mut();
    
    if !egui_init.initialized {
        ctx.set_pixels_per_point(1.5);
        egui_init.initialized = true;
    }

    if state.current_screen == Screen::Share && qr_textures.android_qr.is_none() {
        qr_textures.android_qr = generate_qr_code(ANDROID_DOWNLOAD_URL);
        qr_textures.ios_qr = generate_qr_code(IOS_DOWNLOAD_URL);
    }

    apply_modern_style(ctx);

    if state.current_screen == Screen::Playing && !game.game_over && !game.paused {
        handle_keyboard_input(&keyboard, &mut game);
    }

    if state.current_screen == Screen::Playing {
        run_game_logic(&time, &mut game, &mut profile, &mut leaderboard, &db_res);
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_gray(20)))
        .show(ctx, |ui| {
            match state.current_screen {
                Screen::MainMenu => show_main_menu(ui, &mut state),
                Screen::Playing => show_game_screen(ui, &mut state, &mut game, &profile),
                Screen::Settings => show_settings_screen(ui, &mut state),
                Screen::Leaderboard => show_leaderboard_screen(ui, &mut state, &leaderboard),
                Screen::Profile => show_profile_screen(ui, &mut state, &mut profile, &db_res),
                Screen::Share => show_share_screen(ui, &mut state, &qr_textures),
            }
        });
}