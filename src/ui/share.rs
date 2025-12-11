use eframe::egui;
use qrcode::{QrCode, Color as QrColor};
use crate::resources::{GameState, Screen, QRCodeTextures};
use crate::constants::{ANDROID_DOWNLOAD_URL, IOS_DOWNLOAD_URL};

const TOP_SAFE_AREA: f32 = 24.0;
const BOTTOM_SAFE_AREA: f32 = 24.0;

pub fn generate_qr_textures(ctx: &egui::Context) -> QRCodeTextures {
    let mut textures = QRCodeTextures::default();

    let create_qr = |url: &str, name: &str| -> egui::TextureHandle {
        let code = QrCode::new(url.as_bytes()).unwrap();
        let image = egui::ColorImage {
            size: [code.width(), code.width()],
            pixels: code.to_colors()
                .into_iter()
                .map(|c| if c == QrColor::Dark { egui::Color32::BLACK } else { egui::Color32::WHITE })
                .collect(),
        };
        ctx.load_texture(name, image, egui::TextureOptions::NEAREST)
    };

    textures.android_qr = Some(create_qr(ANDROID_DOWNLOAD_URL, "android_qr"));
    textures.ios_qr = Some(create_qr(IOS_DOWNLOAD_URL, "ios_qr"));

    textures
}

pub fn show_share_screen(ui: &mut egui::Ui, state: &mut GameState, qr_textures: &mut QRCodeTextures, ctx: &egui::Context) {
    if qr_textures.android_qr.is_none() {
        *qr_textures = generate_qr_textures(ctx);
    }

    ui.add_space(TOP_SAFE_AREA);
    
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.add_sized([70.0, 35.0], egui::Button::new(
                egui::RichText::new("Back").size(12.5)
            )).clicked() {
                state.current_screen = Screen::MainMenu;
            }
        });
        
        ui.add_space(15.0);
        ui.heading(egui::RichText::new("Share Game").size(28.0));
        ui.add_space(10.0);
        ui.label(egui::RichText::new("Scan to Download").size(14.0).color(egui::Color32::GRAY));
        ui.add_space(20.0);
        
        ui.group(|ui| {
            ui.set_min_width(220.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Android").size(18.0).strong());
                ui.add_space(10.0);
                
                if let Some(handle) = &qr_textures.android_qr {
                    ui.image((handle.id(), egui::vec2(160.0, 160.0)));
                }
                
                ui.add_space(12.0);
                
                ui.hyperlink_to(
                    egui::RichText::new("Download APK").size(16.0).color(egui::Color32::from_rgb(100, 200, 255)),
                    ANDROID_DOWNLOAD_URL
                );
            });
        });
        
        ui.add_space(20.0);
        
        ui.group(|ui| {
            ui.set_min_width(220.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("iOS").size(18.0).strong());
                ui.add_space(10.0);
                
                if let Some(handle) = &qr_textures.ios_qr {
                    ui.image((handle.id(), egui::vec2(160.0, 160.0)));
                }
                
                ui.add_space(12.0);
                
                ui.hyperlink_to(
                    egui::RichText::new("Download IPA").size(16.0).color(egui::Color32::from_rgb(100, 200, 255)),
                    IOS_DOWNLOAD_URL
                );
            });
        });
        
        ui.add_space(25.0);
        ui.label(egui::RichText::new("Share with friends!")
            .size(16.0)
            .color(egui::Color32::from_rgb(0, 200, 255)));
            
        ui.add_space(BOTTOM_SAFE_AREA);
    });
}