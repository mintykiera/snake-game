use bevy_egui::egui;
use crate::resources::{GameState, Screen, UserProfile};
use crate::constants::*;
use crate::game_logic::save_user_data;
use crate::database::Database;

pub fn show_profile_screen(
    ui: &mut egui::Ui,
    state: &mut GameState,
    profile: &mut UserProfile,
    db: &mut Database,
) {
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.button("Back").clicked() {
                save_user_data(profile, db);
                state.current_screen = Screen::MainMenu;
            }
        });
        
        ui.add_space(25.0);
        
        ui.heading("Profile");
        
        ui.add_space(30.0);
        
        ui.group(|ui| {
            ui.set_min_width(300.0);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Username").color(egui::Color32::GRAY));
                ui.add_space(5.0);
                ui.text_edit_singleline(&mut profile.username);
            });
        });
        
        ui.add_space(15.0);
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("High Score: {}", profile.high_score)).color(egui::Color32::from_rgb(0, 255, 100)));
        });
        
        ui.add_space(10.0);
        
        ui.label(egui::RichText::new(format!("ID: {}", &profile.user_id[..8])).color(egui::Color32::GRAY));
        
        ui.add_space(35.0);
        
        ui.heading("Customization");
        
        ui.add_space(20.0);
        
        ui.group(|ui| {
            ui.set_min_width(300.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Snake");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut color = [
                            profile.snake_color[0] as f32 / 255.0,
                            profile.snake_color[1] as f32 / 255.0,
                            profile.snake_color[2] as f32 / 255.0,
                        ];
                        if egui::color_picker::color_edit_button_rgb(ui, &mut color).changed() {
                            profile.snake_color = [
                                (color[0] * 255.0) as u8,
                                (color[1] * 255.0) as u8,
                                (color[2] * 255.0) as u8,
                            ];
                        }
                    });
                });
                
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label("Background");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut color = [
                            profile.background_color[0] as f32 / 255.0,
                            profile.background_color[1] as f32 / 255.0,
                            profile.background_color[2] as f32 / 255.0,
                        ];
                        if egui::color_picker::color_edit_button_rgb(ui, &mut color).changed() {
                            profile.background_color = [
                                (color[0] * 255.0) as u8,
                                (color[1] * 255.0) as u8,
                                (color[2] * 255.0) as u8,
                            ];
                        }
                    });
                });
                
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label("Apple");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut color = [
                            profile.apple_color[0] as f32 / 255.0,
                            profile.apple_color[1] as f32 / 255.0,
                            profile.apple_color[2] as f32 / 255.0,
                        ];
                        if egui::color_picker::color_edit_button_rgb(ui, &mut color).changed() {
                            profile.apple_color = [
                                (color[0] * 255.0) as u8,
                                (color[1] * 255.0) as u8,
                                (color[2] * 255.0) as u8,
                            ];
                        }
                    });
                });
            });
        });
        
        ui.add_space(20.0);
        
        ui.horizontal(|ui| {
            let available_width = ui.available_width();
            let button_width = 140.0;
            let spacing = 10.0;
            let total_width = button_width * 2.0 + spacing;
            let padding = (available_width - total_width) / 2.0;
            
            ui.add_space(padding.max(0.0));
            
            if ui.add_sized([button_width, 42.0], egui::Button::new("Reset Colors")).clicked() {
                profile.snake_color = DEFAULT_SNAKE_COLOR;
                profile.background_color = DEFAULT_BACKGROUND_COLOR;
                profile.apple_color = DEFAULT_APPLE_COLOR;
            }
            
            ui.add_space(spacing);
            
            if ui.add_sized([button_width, 42.0], egui::Button::new("Save Profile")).clicked() {
                save_user_data(profile, db);
            }
        });
    });
}