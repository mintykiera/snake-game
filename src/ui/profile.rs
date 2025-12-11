use eframe::egui;
use crate::resources::{GameState, Screen, UserProfile};
use crate::constants::*;
use crate::ui::components;

const TOP_SAFE_AREA: f32 = 24.0;

#[cfg(not(target_os = "android"))]
pub fn show_profile_screen(
    ui: &mut egui::Ui,
    state: &mut GameState,
    profile: &mut UserProfile,
) -> bool {
    let mut request_sync = false;

    ui.add_space(TOP_SAFE_AREA);

    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui
                .add_sized(
                    [70.0, 35.0],
                    egui::Button::new(egui::RichText::new("Back").size(12.5)),
                )
                .clicked()
            {
                state.current_screen = Screen::MainMenu;
            }
        });

        ui.add_space(20.0);
        ui.heading(egui::RichText::new("Profile").size(28.0));
        ui.add_space(25.0);

        ui.group(|ui| {
            ui.set_width(ui.available_width());
            
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Username")
                            .size(14.0)
                            .color(egui::Color32::GRAY),
                    );
                });
                
                ui.add_space(5.0);

                let response = ui.add_sized(
                    [ui.available_width(), 35.0],
                    egui::TextEdit::singleline(&mut profile.username).hint_text("Tap to set name"),
                );

                if response.lost_focus() {
                    request_sync = true;
                }
            });
        });

        ui.add_space(15.0);

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("High Score: {}", profile.high_score))
                    .size(18.0)
                    .color(egui::Color32::from_rgb(0, 255, 100)),
            );
        });

        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(format!(
                "ID: {}",
                &profile.user_id[..8.min(profile.user_id.len())]
            ))
            .size(12.0)
            .color(egui::Color32::GRAY),
        );

        ui.add_space(30.0);
        ui.heading(egui::RichText::new("Customization").size(22.0));
        ui.add_space(20.0);

        ui.group(|ui| {
            ui.set_width(ui.available_width());

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Snake").size(16.0));
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
                    ui.label(egui::RichText::new("Background").size(16.0));
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
                    ui.label(egui::RichText::new("Apple").size(16.0));
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

        ui.add_space(25.0);

        if ui
            .add_sized(
                [160.0, 45.0],
                egui::Button::new(egui::RichText::new("Reset Colors").size(14.0)),
            )
            .clicked()
        {
            profile.snake_color = DEFAULT_SNAKE_COLOR;
            profile.background_color = DEFAULT_BACKGROUND_COLOR;
            profile.apple_color = DEFAULT_APPLE_COLOR;
        }

        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Changes are saved automatically")
                .size(12.0)
                .color(egui::Color32::GRAY),
        );

        ui.add_space(BOTTOM_SAFE_AREA);
    });

    request_sync
}

#[cfg(target_os = "android")]
pub fn show_profile_screen(
    ui: &mut egui::Ui,
    state: &mut GameState,
    profile: &mut UserProfile,
    show_keyboard: &mut bool,
) -> bool {
    let mut request_sync = false;

    ui.add_space(TOP_SAFE_AREA);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        [70.0, 35.0],
                        egui::Button::new(egui::RichText::new("Back").size(12.5)),
                    )
                    .clicked()
                {
                    state.current_screen = Screen::MainMenu;
                    *show_keyboard = false;
                }
            });

            ui.add_space(20.0);
            ui.heading(egui::RichText::new("Profile").size(28.0));
            ui.add_space(25.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Username")
                                .size(14.0)
                                .color(egui::Color32::GRAY),
                        );
                    });
                    ui.add_space(5.0);

                    let display_name = if profile.username.is_empty() {
                        "Tap to set name".to_string()
                    } else {
                        profile.username.clone()
                    };

                    if ui
                        .add_sized(
                            [ui.available_width(), 35.0],
                            egui::Button::new(egui::RichText::new(&display_name).size(16.0)),
                        )
                        .clicked()
                    {
                        *show_keyboard = !*show_keyboard;
                    }
                });
            });

            ui.add_space(15.0);

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("High Score: {}", profile.high_score))
                        .size(18.0)
                        .color(egui::Color32::from_rgb(0, 255, 100)),
                );
            });

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(format!(
                    "ID: {}",
                    &profile.user_id[..8.min(profile.user_id.len())]
                ))
                .size(12.0)
                .color(egui::Color32::GRAY),
            );

            ui.add_space(30.0);
            ui.heading(egui::RichText::new("Customization").size(22.0));
            ui.add_space(20.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Snake").size(16.0));
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
                        ui.label(egui::RichText::new("Background").size(16.0));
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
                        ui.label(egui::RichText::new("Apple").size(16.0));
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

            ui.add_space(25.0);

            if ui
                .add_sized(
                    [160.0, 45.0],
                    egui::Button::new(egui::RichText::new("Reset Colors").size(14.0)),
                )
                .clicked()
            {
                profile.snake_color = DEFAULT_SNAKE_COLOR;
                profile.background_color = DEFAULT_BACKGROUND_COLOR;
                profile.apple_color = DEFAULT_APPLE_COLOR;
            }

            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("Changes are saved automatically")
                    .size(12.0)
                    .color(egui::Color32::GRAY),
            );

            ui.add_space(300.0);
        });
    });

    if components::keyboard::show(ui.ctx(), show_keyboard, &mut profile.username) {
        request_sync = true;
    }

    request_sync
}