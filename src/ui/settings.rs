use bevy_egui::egui;
use crate::resources::{GameState, Screen};

pub fn show_settings_screen(ui: &mut egui::Ui, state: &mut GameState) {
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.button("Back").clicked() {
                state.current_screen = Screen::MainMenu;
            }
        });
        
        ui.add_space(60.0);
        
        ui.heading("Settings");
        
        ui.add_space(100.0);
        
        ui.label(egui::RichText::new("Coming Soon").size(24.0).color(egui::Color32::from_rgb(255, 200, 0)));
        
        ui.add_space(30.0);
        
        ui.label(egui::RichText::new("Future features:").color(egui::Color32::GRAY));
        ui.add_space(10.0);
        ui.label("• Sound effects");
        ui.label("• Game speed settings");
        ui.label("• Snake skins");
        ui.label("• Difficulty modes");
    });
}