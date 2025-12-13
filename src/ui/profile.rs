use eframe::egui;
use crate::resources::{GameState, Screen, UserProfile};
use crate::constants::*;
use crate::ui::components;

const TOP_SAFE_AREA: f32 = 24.0;
const BOTTOM_SAFE_AREA: f32 = 24.0;

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
            if draw_shared_profile_ui(ui, state, profile, Some(show_keyboard)) {
                request_sync = true;
            }
            ui.add_space(300.0);
        });
    });

    if components::keyboard::show(ui.ctx(), show_keyboard, &mut profile.username) {
        request_sync = true;
    }

    request_sync
}

#[cfg(not(target_os = "android"))]
pub fn show_profile_screen(
    ui: &mut egui::Ui,
    state: &mut GameState,
    profile: &mut UserProfile,
) -> bool {
    ui.add_space(TOP_SAFE_AREA);
    
    let mut request_sync = false;
    ui.vertical_centered(|ui| {
        if draw_shared_profile_ui(ui, state, profile, None) {
            request_sync = true;
        }
    });
    request_sync
}

fn draw_shared_profile_ui(
    ui: &mut egui::Ui,
    state: &mut GameState,
    profile: &mut UserProfile,
    mut mobile_keyboard_trigger: Option<&mut bool>,
) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        if ui.add_sized([70.0, 35.0], egui::Button::new(egui::RichText::new("Back").size(12.5))).clicked() {
            state.current_screen = Screen::MainMenu;
            if let Some(trigger) = mobile_keyboard_trigger.as_deref_mut() {
                *trigger = false; 
            }
        }
    });

    ui.add_space(20.0);
    ui.heading(egui::RichText::new("Profile").size(28.0));
    ui.add_space(25.0);

    ui.group(|ui| {
        ui.set_width(ui.available_width());
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Username").size(14.0).color(egui::Color32::GRAY));
            });
            ui.add_space(5.0);

            let btn_size = [ui.available_width(), 35.0];

            if let Some(trigger) = mobile_keyboard_trigger {
                let display_name = if profile.username.is_empty() { "Tap to set name" } else { &profile.username };
                if ui.add_sized(btn_size, egui::Button::new(egui::RichText::new(display_name).size(16.0))).clicked() {
                    *trigger = !*trigger;
                }
            } else {
                let response = ui.add_sized(btn_size, egui::TextEdit::singleline(&mut profile.username).hint_text("Tap to set name"));
                if response.lost_focus() {
                    changed = true;
                }
            }
        });
    });

    ui.add_space(15.0);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(format!("High Score: {}", profile.high_score))
            .size(18.0).color(egui::Color32::from_rgb(0, 255, 100)));
    });

    ui.add_space(10.0);
    ui.label(egui::RichText::new(format!("ID: {}", &profile.user_id[..8.min(profile.user_id.len())]))
        .size(12.0).color(egui::Color32::GRAY));

    ui.add_space(30.0);
    ui.heading(egui::RichText::new("Customization").size(22.0));
    ui.add_space(20.0);

    ui.group(|ui| {
        ui.set_width(ui.available_width());
        ui.vertical(|ui| {
            if color_row(ui, "Snake", &mut profile.snake_color) { changed = true; }
            ui.add_space(12.0);
            
            if color_row(ui, "Background", &mut profile.background_color) { changed = true; }
            ui.add_space(12.0);
            
            if color_row(ui, "Apple", &mut profile.apple_color) { changed = true; }
        });
    });

    ui.add_space(25.0);

    if ui.add_sized([160.0, 45.0], egui::Button::new(egui::RichText::new("Reset Colors").size(14.0))).clicked() {
        profile.snake_color = DEFAULT_SNAKE_COLOR;
        profile.background_color = DEFAULT_BACKGROUND_COLOR;
        profile.apple_color = DEFAULT_APPLE_COLOR;
        changed = true;
    }

    ui.add_space(10.0);
    ui.label(egui::RichText::new("Changes are saved automatically").size(12.0).color(egui::Color32::GRAY));
    
    ui.add_space(BOTTOM_SAFE_AREA); 

    changed
}

fn color_row(ui: &mut egui::Ui, label: &str, color_arr: &mut [u8; 3]) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).size(16.0));
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mut rgb = [
                color_arr[0] as f32 / 255.0,
                color_arr[1] as f32 / 255.0,
                color_arr[2] as f32 / 255.0,
            ];
            
            if egui::color_picker::color_edit_button_rgb(ui, &mut rgb).changed() {
                *color_arr = [
                    (rgb[0] * 255.0) as u8,
                    (rgb[1] * 255.0) as u8,
                    (rgb[2] * 255.0) as u8,
                ];
                changed = true;
            }
        });
    });
    changed
}