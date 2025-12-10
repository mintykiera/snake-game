use eframe::egui;
use crate::resources::{GameState, Screen, Leaderboard};

const TOP_SAFE_AREA: f32 = 24.0;
const BOTTOM_SAFE_AREA: f32 = 24.0;

pub fn show_leaderboard_screen(ui: &mut egui::Ui, state: &mut GameState, leaderboard: &Leaderboard) -> bool {
    let mut request_refresh = false;

    ui.add_space(TOP_SAFE_AREA);
    
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.add_sized([70.0, 35.0], egui::Button::new(
                egui::RichText::new("Back").size(12.5)
            )).clicked() {
                state.current_screen = Screen::MainMenu;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add_sized([80.0, 35.0], egui::Button::new(
                    egui::RichText::new("Refresh").size(12.5)
                )).clicked() {
                    request_refresh = true;
                }
            });
        });
        
        ui.add_space(20.0);
        
        ui.heading(egui::RichText::new("Leaderboard").size(28.0));
        
        ui.add_space(30.0);
        
        if leaderboard.entries.is_empty() {
            ui.label(egui::RichText::new("No scores yet").size(18.0).color(egui::Color32::GRAY));
            ui.add_space(15.0);
            ui.label(egui::RichText::new("Play to set your first record!").size(16.0));
            ui.add_space(10.0);
            if ui.button("Force Refresh").clicked() {
                request_refresh = true;
            }
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, entry) in leaderboard.entries.iter().take(10).enumerate() {
                    ui.horizontal(|ui| {
                        let rank_color = match i {
                            0 => egui::Color32::from_rgb(255, 215, 0),
                            1 => egui::Color32::from_rgb(192, 192, 192),
                            2 => egui::Color32::from_rgb(205, 127, 50),
                            _ => egui::Color32::GRAY,
                        };

                        ui.label(egui::RichText::new(format!("#{}", i + 1))
                            .size(18.0)
                            .color(rank_color)
                            .strong());
                        
                        ui.add_space(10.0);
                        
                        ui.label(egui::RichText::new(&entry.username).size(16.0));
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(format!("{}", entry.score))
                                .size(18.0)
                                .color(egui::Color32::from_rgb(0, 255, 100)));
                        });
                    });
                    ui.add_space(12.0);
                }
            });
        }
        
        ui.add_space(BOTTOM_SAFE_AREA);
    });

    request_refresh
}