mod game_screen;
mod leaderboard;
mod main_menu;
mod profile;
mod settings;
mod share;

use game_screen::show_game_screen;
use leaderboard::show_leaderboard_screen;
use main_menu::show_main_menu;
use profile::show_profile_screen;
use settings::show_settings_screen;
use share::show_share_screen;

use bevy::prelude::*;
use bevy::input::Input;
use bevy_egui::{egui, EguiContexts};
use crate::resources::*;
use crate::game_logic::*;
use crate::database::Database;

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
    qr_textures: Res<QRCodeTextures>,
    keyboard: Res<Input<KeyCode>>,
    db_res: Res<Database>,
) {
    let ctx = contexts.ctx_mut();
    
    if !egui_init.initialized {
        ctx.set_pixels_per_point(1.5);
        egui_init.initialized = true;
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