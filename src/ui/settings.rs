use eframe::egui;
use crate::resources::{GameState, Screen};

const TOP_SAFE_AREA: f32 = 24.0;
const BOTTOM_SAFE_AREA: f32 = 24.0;

pub fn show_settings_screen(ui: &mut egui::Ui, state: &mut GameState) {
    ui.add_space(TOP_SAFE_AREA);
    
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.add_sized([70.0, 35.0], egui::Button::new(
                egui::RichText::new("Back").size(12.5)
            )).clicked() {
                state.current_screen = Screen::MainMenu;
            }
        });
        
        ui.add_space(40.0);
        
        ui.heading(egui::RichText::new("Settings").size(28.0));
        
        ui.add_space(80.0);
        
        ui.label(egui::RichText::new("Coming Soon")
            .size(28.0)
            .color(egui::Color32::from_rgb(255, 200, 0)));
        
        ui.add_space(40.0);
        
        ui.label(egui::RichText::new("Future features:")
            .size(16.0)
            .color(egui::Color32::GRAY));
        ui.add_space(15.0);
        ui.label(egui::RichText::new("• Sound effects").size(16.0));
        ui.add_space(8.0);
        ui.label(egui::RichText::new("• Game speed settings").size(16.0));
        ui.add_space(8.0);
        ui.label(egui::RichText::new("• Snake skins").size(16.0));
        ui.add_space(8.0);
        ui.label(egui::RichText::new("• Difficulty modes").size(16.0));
        
        ui.add_space(BOTTOM_SAFE_AREA);
    });
}