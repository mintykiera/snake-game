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
        
        ui.heading("Share Game");
        
        ui.add_space(15.0);
        
        ui.label(egui::RichText::new("Scan to Download").color(egui::Color32::GRAY));
        
        ui.add_space(25.0);
        
        ui.label(egui::RichText::new("Android").strong());
        ui.add_space(8.0);
        
        if let Some(texture_id) = qr_textures.android_qr {
            ui.image((texture_id, egui::vec2(128.0, 128.0)));
        } else {
            ui.label("Generating QR code...");
        }
        
        ui.add_space(8.0);
        ui.label(egui::RichText::new("https://github.com/mintykiera/snake-game/releases/tag/v1.0snake_game.apk").color(egui::Color32::GRAY));
        
        ui.add_space(25.0);
        
        ui.label(egui::RichText::new("iOS ( no ios yet btw :'c )").strong());
        ui.add_space(8.0);
        
        if let Some(texture_id) = qr_textures.ios_qr {
            ui.image((texture_id, egui::vec2(128.0, 128.0)));
        } else {
            ui.label("Generating QR code...");
        }
        
        ui.add_space(8.0);
        ui.label(egui::RichText::new("https://github.com/mintykiera/snake-game/releases/tag/v1.0snake_game.apk").color(egui::Color32::GRAY));
        // we're gonna change this to ipa once we have that working...
        
        ui.add_space(20.0);
        
        ui.label(egui::RichText::new("Share with friends!").color(egui::Color32::from_rgb(0, 200, 255)));
    });
}