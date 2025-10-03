use bevy_egui::egui;
use crate::resources::{GameState, Screen, QRCodeTextures};

pub fn show_share_screen(ui: &mut egui::Ui, state: &mut GameState, qr_textures: &QRCodeTextures) {
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.button("Back").clicked() {
                state.current_screen = Screen::MainMenu;
            }
        });
        
        ui.add_space(20.0);
        
        ui.heading(egui::RichText::new("Share Game").size(28.0));
        
        ui.add_space(15.0);
        
        ui.label(egui::RichText::new("Scan to Download").size(14.0).color(egui::Color32::GRAY));
        
        ui.add_space(25.0);
        
        ui.label(egui::RichText::new("Android").size(16.0).strong());
        ui.add_space(8.0);
        
        if let Some(qr_string) = &qr_textures.android_qr {
            ui.label(egui::RichText::new(qr_string).monospace().size(8.0));
        } else {
            ui.label("Generating QR code...");
        }
        
        ui.add_space(8.0);
        ui.label(egui::RichText::new("github.com/yourusername/snake-android").size(9.0).color(egui::Color32::GRAY));
        
        ui.add_space(25.0);
        
        ui.label(egui::RichText::new("iOS").size(16.0).strong());
        ui.add_space(8.0);
        
        if let Some(qr_string) = &qr_textures.ios_qr {
            ui.label(egui::RichText::new(qr_string).monospace().size(8.0));
        } else {
            ui.label("Generating QR code...");
        }
        
        ui.add_space(8.0);
        ui.label(egui::RichText::new("github.com/yourusername/snake-ios").size(9.0).color(egui::Color32::GRAY));
        
        ui.add_space(20.0);
        
        ui.label(egui::RichText::new("Share with friends!").size(12.0).color(egui::Color32::from_rgb(0, 200, 255)));
    });
}