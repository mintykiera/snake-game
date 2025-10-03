use bevy_egui::egui;
use crate::resources::{GameState, Screen};

pub fn show_main_menu(ui: &mut egui::Ui, state: &mut GameState) {
    ui.vertical_centered(|ui| {
        ui.add_space(60.0);
        
        ui.heading(egui::RichText::new("SNAKE").size(48.0).color(egui::Color32::from_rgb(0, 255, 100)));
        ui.label(egui::RichText::new("Classic Reimagined").color(egui::Color32::GRAY));
        
        ui.add_space(100.0);
        
        let button_size = egui::vec2(240.0, 55.0);
        
        if ui.add_sized(button_size, egui::Button::new("Play")).clicked() {
            state.current_screen = Screen::Playing;
        }
        
        ui.add_space(15.0);
        
        if ui.add_sized(button_size, egui::Button::new("Settings")).clicked() {
            state.current_screen = Screen::Settings;
        }
        
        ui.add_space(15.0);
        
        if ui.add_sized(button_size, egui::Button::new("Leaderboard")).clicked() {
            state.current_screen = Screen::Leaderboard;
        }
        
        ui.add_space(15.0);
        
        if ui.add_sized(button_size, egui::Button::new("Profile")).clicked() {
            state.current_screen = Screen::Profile;
        }
        
        ui.add_space(15.0);
        
        if ui.add_sized(button_size, egui::Button::new("Share")).clicked() {
            state.current_screen = Screen::Share;
        }
    });
}