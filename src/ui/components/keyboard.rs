use eframe::egui;

const KEY_HEIGHT: f32 = 48.0;
const KEY_ROUNDING: f32 = 8.0;
const SPACING: f32 = 5.0;
const SIDE_MARGIN: f32 = 12.0;

fn key_color_normal() -> egui::Color32 { egui::Color32::from_gray(60) }
fn key_color_special() -> egui::Color32 { egui::Color32::from_gray(45) }
fn key_color_submit() -> egui::Color32 { egui::Color32::from_rgb(0, 100, 200) }
fn key_color_active() -> egui::Color32 { egui::Color32::from_rgb(0, 120, 215) }

#[derive(Clone, Copy, PartialEq)]
enum ShiftState {
    Off,
    Shift,
    CapsLock,
}

pub fn show(ctx: &egui::Context, show: &mut bool, text: &mut String) -> bool {
    let mut changed = false;

    let t = ctx.animate_bool_with_time(egui::Id::new("keyboard_anim"), *show, 0.25);
    if t == 0.0 { return false; }

    let state_id = egui::Id::new("keyboard_state");
    
    let mut shift_state: ShiftState = ctx.data(|d| d.get_temp(state_id).unwrap_or(ShiftState::Off));
    
    let last_shift_click_id = egui::Id::new("last_shift_click");
    let last_shift_time: f64 = ctx.data(|d| d.get_temp(last_shift_click_id).unwrap_or(0.0));

    let backspace_timer_id = egui::Id::new("backspace_timer");
    let mut backspace_start: Option<f64> = ctx.data(|d| d.get_temp(backspace_timer_id));

    let rows = 5; 
    let total_height = (KEY_HEIGHT * rows as f32) + (SPACING * (rows as f32 + 2.0)) + 30.0;
    let screen_rect = ctx.screen_rect();
    let current_y = screen_rect.height() - (total_height * t);

    egui::Area::new(egui::Id::new("keyboard_area"))
        .fixed_pos(egui::pos2(0.0, current_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            
            let frame = egui::Frame::none()
                .fill(egui::Color32::from_black_alpha(245))
                .rounding(egui::Rounding { nw: 16.0, ne: 16.0, sw: 0.0, se: 0.0 })
                .inner_margin(egui::Margin::symmetric(SIDE_MARGIN, 10.0));

            frame.show(ui, |ui| {
                ui.set_width(screen_rect.width() - (SIDE_MARGIN * 2.0)); 
                ui.set_height(total_height);

                let available_width = ui.available_width();
                let base_key_width = (available_width - (SPACING * 9.0)) / 10.0;
                let standard_size = egui::vec2(base_key_width, KEY_HEIGHT);

                ui.spacing_mut().item_spacing = egui::vec2(SPACING, SPACING);

                ui.vertical(|ui| {
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        for c in "1234567890".chars() {
                            if key_btn(ui, c.to_string(), standard_size, key_color_normal(), false).clicked() {
                                text.push(c);
                                changed = true;
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        for c in "QWERTYUIOP".chars() {
                            let char_str = if shift_state != ShiftState::Off { c.to_string() } else { c.to_lowercase().to_string() };
                            
                            if key_btn(ui, char_str.clone(), standard_size, key_color_normal(), false).clicked() {
                                text.push_str(&char_str);
                                changed = true;
                                
                                if shift_state == ShiftState::Shift {
                                    shift_state = ShiftState::Off;
                                }
                            }
                        }
                    });

                    ui.horizontal(|ui| {

                        ui.add_space(base_key_width * 0.4); 

                        for c in "ASDFGHJKL".chars() {
                            let char_str = if shift_state != ShiftState::Off { c.to_string() } else { c.to_lowercase().to_string() };
                            if key_btn(ui, char_str.clone(), standard_size, key_color_normal(), false).clicked() {
                                text.push_str(&char_str);
                                changed = true;
                                if shift_state == ShiftState::Shift {
                                    shift_state = ShiftState::Off;
                                }
                            }
                        }
                    });

                    ui.horizontal(|ui| {

                        let shift_width = base_key_width * 1.3;
                        let letter_keys_width = (base_key_width * 7.0) + (SPACING * 7.0);
                        let back_width = available_width - shift_width - letter_keys_width;

                        let (shift_icon, shift_color, text_color) = match shift_state {
                            ShiftState::Off => ("⬆", key_color_special(), egui::Color32::WHITE),
                            ShiftState::Shift => ("⬆", key_color_active().linear_multiply(0.5), egui::Color32::WHITE),
                            ShiftState::CapsLock => ("⇪", key_color_active(), egui::Color32::WHITE),
                        };

                        if ui.add_sized(
                            egui::vec2(shift_width, KEY_HEIGHT),
                            egui::Button::new(egui::RichText::new(shift_icon).size(16.0).strong().color(text_color))
                                .fill(shift_color)
                                .rounding(KEY_ROUNDING)
                        ).clicked() {
                            let now = ui.input(|i| i.time);
                            if now - last_shift_time < 0.3 {
                                shift_state = ShiftState::CapsLock;
                            } else {
                                shift_state = match shift_state {
                                    ShiftState::Off => ShiftState::Shift,
                                    _ => ShiftState::Off,
                                };
                            }
                            ctx.data_mut(|d| d.insert_temp(last_shift_click_id, now));
                        }

                        for c in "ZXCVBNM".chars() {
                            let char_str = if shift_state != ShiftState::Off { c.to_string() } else { c.to_lowercase().to_string() };
                            if key_btn(ui, char_str.clone(), standard_size, key_color_normal(), false).clicked() {
                                text.push_str(&char_str);
                                changed = true;
                                if shift_state == ShiftState::Shift {
                                    shift_state = ShiftState::Off;
                                }
                            }
                        }

                        let back_btn = key_btn(ui, "BACK".to_string(), egui::vec2(back_width, KEY_HEIGHT), key_color_special(), true);
                        
                        if back_btn.clicked() {
                            text.pop();
                            changed = true;
                        }

                        if back_btn.is_pointer_button_down_on() {
                            let now = ui.input(|i| i.time);
                            if let Some(start_time) = backspace_start {
                                if now - start_time > 0.5 {
                                    text.pop();
                                    changed = true;
                                }
                            } else {
                                backspace_start = Some(now);
                            }
                        } else {
                            backspace_start = None;
                        }
                    });

                    ui.horizontal(|ui| {
                        let total_w = ui.available_width();

                        let spacer_w = total_w * 0.20;
                        let space_w = total_w * 0.60;
                        let done_w = total_w * 0.15;

                        ui.add_space(spacer_w);

                        if key_btn(ui, "SPACE".to_string(), egui::vec2(space_w, KEY_HEIGHT), key_color_normal(), false).clicked() {
                            text.push(' ');
                            changed = true;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                             if key_btn(ui, "DONE".to_string(), egui::vec2(done_w, KEY_HEIGHT), key_color_submit(), true).clicked() {
                                *show = false;
                            }
                        });
                    });

                    ui.add_space(20.0);
                });
            });
        });

    ctx.data_mut(|d| {
        d.insert_temp(state_id, shift_state);
        d.insert_temp(backspace_timer_id, backspace_start);
    });

    changed
}

fn key_btn(ui: &mut egui::Ui, label: String, size: egui::Vec2, bg_color: egui::Color32, bold: bool) -> egui::Response {
    let mut text = egui::RichText::new(label);
    
    if text.text().len() > 1 {
        text = text.size(13.0).strong();
    } else {
        text = text.size(20.0);
    }

    if bold { text = text.strong(); }

    ui.add_sized(
        size,
        egui::Button::new(text.color(egui::Color32::WHITE))
            .fill(bg_color)
            .rounding(KEY_ROUNDING)
    )
}