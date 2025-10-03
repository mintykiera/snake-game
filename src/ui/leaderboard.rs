use bevy_egui::egui;
use crate::resources::{GameState, Screen, Leaderboard};

pub fn show_leaderboard_screen(ui: &mut egui::Ui, state: &mut GameState, leaderboard: &Leaderboard) {
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.button("Back").clicked() {
                state.current_screen = Screen::MainMenu;
            }
        });
        
        ui.add_space(30.0);
        
        ui.heading("Leaderboard");
        
        ui.add_space(40.0);
        
        if leaderboard.entries.is_empty() {
            ui.label(egui::RichText::new("No scores yet").color(egui::Color32::GRAY));
            ui.add_space(15.0);
            ui.label("Play to set your first record!");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, entry) in leaderboard.entries.iter().take(10).enumerate() {
                    ui.horizontal(|ui| {
                        let medal = match i {
                            0 => "🥇",
                            1 => "🥈",
                            2 => "🥉",
                            _ => "  ",
                        };
                        ui.label(format!("{} #{}", medal, i + 1));
                        ui.label(&entry.username);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(format!("{}", entry.score)).color(egui::Color32::from_rgb(0, 255, 100)));
                        });
                    });
                    ui.add_space(12.0);
                }
            });
        }
    });
}