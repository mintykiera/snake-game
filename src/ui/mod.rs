pub mod game_screen;
pub mod leaderboard;
pub mod main_menu;
pub mod profile;
pub mod settings;
pub mod share;

use eframe::egui;

pub fn apply_modern_style(ctx: &egui::Context) {
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