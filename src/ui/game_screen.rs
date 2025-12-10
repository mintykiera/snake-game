use eframe::egui;
use crate::resources::{Game, GameState, Screen, UserProfile, Direction};
use crate::constants::{GRID_SIZE, CELL_SIZE};

const TOP_SAFE_AREA: f32 = 24.0;
const BOTTOM_SAFE_AREA: f32 = 24.0;

pub fn show_game_screen(
    ui: &mut egui::Ui,
    state: &mut GameState,
    game: &mut Game,
    profile: &UserProfile,
) {
    let screen_width = ui.available_width();
    let screen_height = ui.available_height();
    let is_landscape = screen_width > screen_height;

    if is_landscape {
        show_game_screen_landscape(ui, state, game, profile);
    } else {
        show_game_screen_portrait(ui, state, game, profile);
    }
}

fn show_game_screen_portrait(
    ui: &mut egui::Ui,
    state: &mut GameState,
    game: &mut Game,
    profile: &UserProfile,
) {
    ui.add_space(TOP_SAFE_AREA);
    
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            if ui.add_sized([70.0, 35.0], egui::Button::new(
                egui::RichText::new("Back").size(12.5)
            )).clicked() {
                state.current_screen = Screen::MainMenu;
                *game = Game::default();
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new(format!("Best: {}", profile.high_score))
                    .size(16.0)
                    .color(egui::Color32::GRAY));
            });
        });
        
        ui.add_space(12.0);
        
        ui.heading(egui::RichText::new(format!("Score: {}", game.score))
            .size(28.0)
            .color(egui::Color32::WHITE));
        
        ui.add_space(15.0);
        
        draw_game_canvas(ui, game, profile);
        
        if game.game_over {
            ui.add_space(30.0);
            ui.colored_label(
                egui::Color32::from_rgb(255, 100, 100),
                egui::RichText::new("Game Over").size(28.0)
            );
            ui.add_space(20.0);
            if ui.add_sized([200.0, 55.0], egui::Button::new(
                egui::RichText::new("Play Again").size(18.0)
            )).clicked() {
                *game = Game::default();
            }
        } else {
            // Calculate space to push dpad toward bottom
            let dpad_total_height = 70.0 * 3.0 + 8.0 * 2.0 + 20.0 + 45.0 + BOTTOM_SAFE_AREA;
            let remaining = ui.available_height() - dpad_total_height;
            
            // Push dpad down, but leave some space at the very bottom
            if remaining > 0.0 {
                ui.add_space(remaining * 0.7);
            } else {
                ui.add_space(20.0);
            }
            
            // Center the dpad
            ui.horizontal(|ui| {
                let dpad_width = 70.0 * 3.0 + 8.0 * 2.0;
                let available_width = ui.available_width();
                let padding = (available_width - dpad_width) / 2.0;
                
                ui.add_space(padding.max(0.0));
                
                draw_dpad_controls(ui, game);
            });
            
            ui.add_space(15.0);
            
            if ui.add_sized([120.0, 45.0], egui::Button::new(
                egui::RichText::new(if game.paused { "Resume" } else { "Pause" }).size(16.0)
            )).clicked() {
                game.paused = !game.paused;
            }
            
            // Bottom safe area
            ui.add_space(BOTTOM_SAFE_AREA);
        }
    });
}

fn show_game_screen_landscape(
    ui: &mut egui::Ui,
    state: &mut GameState,
    game: &mut Game,
    profile: &UserProfile,
) {
    // Side safe areas for landscape
    ui.add_space(TOP_SAFE_AREA);
    
    ui.horizontal(|ui| {
        ui.add_space(24.0); // Left safe area
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // Bigger back button
                if ui.add_sized([100.0, 50.0], egui::Button::new(
                    egui::RichText::new("← Back").size(18.0)
                )).clicked() {
                    state.current_screen = Screen::MainMenu;
                    *game = Game::default();
                }
            });
            
            ui.add_space(8.0);
            ui.heading(egui::RichText::new(format!("Score: {}", game.score)).size(24.0));
            ui.label(egui::RichText::new(format!("Best: {}", profile.high_score)).size(14.0));
            ui.add_space(15.0);
            
            draw_game_canvas(ui, game, profile);
        });
        
        ui.add_space(40.0);
        
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            
            if game.game_over {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 100, 100),
                    egui::RichText::new("Game Over").size(24.0)
                );
                ui.add_space(20.0);
                if ui.add_sized([160.0, 50.0], egui::Button::new(
                    egui::RichText::new("Play Again").size(16.0)
                )).clicked() {
                    *game = Game::default();
                }
            } else {
                draw_dpad_controls(ui, game);
                
                ui.add_space(25.0);
                
                if ui.add_sized([120.0, 45.0], egui::Button::new(
                    egui::RichText::new(if game.paused { "Resume" } else { "Pause" }).size(16.0)
                )).clicked() {
                    game.paused = !game.paused;
                }
            }
        });
        
        ui.add_space(24.0); // Right safe area
    });
}

fn draw_game_canvas(ui: &mut egui::Ui, game: &Game, profile: &UserProfile) {
    let canvas_size = GRID_SIZE as f32 * CELL_SIZE;
    let (response, painter) = ui.allocate_painter(
        egui::vec2(canvas_size, canvas_size),
        egui::Sense::hover(),
    );
    let rect = response.rect;
    
    painter.rect_filled(
        rect,
        4.0,
        egui::Color32::from_rgb(
            profile.background_color[0],
            profile.background_color[1],
            profile.background_color[2],
        ),
    );
    
    painter.rect_stroke(
        rect,
        4.0,
        egui::Stroke::new(2.0, egui::Color32::from_gray(60)),
    );

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = rect.min.x + i as f32 * CELL_SIZE;
            let y = rect.min.y + j as f32 * CELL_SIZE;
            painter.rect_stroke(
                egui::Rect::from_min_size(
                    egui::pos2(x, y),
                    egui::vec2(CELL_SIZE, CELL_SIZE),
                ),
                0.0,
                egui::Stroke::new(0.3, egui::Color32::from_gray(40)),
            );
        }
    }

    for (i, &(x, y)) in game.snake.iter().enumerate() {
        let px = rect.min.x + x as f32 * CELL_SIZE;
        let py = rect.min.y + y as f32 * CELL_SIZE;
        let mut color = profile.snake_color;
        if i == 0 {
            color = [
                color[0].saturating_add(55),
                color[1].saturating_add(55),
                color[2].saturating_add(55),
            ];
        }
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(px + 1.5, py + 1.5),
                egui::vec2(CELL_SIZE - 3.0, CELL_SIZE - 3.0),
            ),
            3.0,
            egui::Color32::from_rgb(color[0], color[1], color[2]),
        );
    }

    let fx = rect.min.x + game.food.0 as f32 * CELL_SIZE;
    let fy = rect.min.y + game.food.1 as f32 * CELL_SIZE;
    painter.circle_filled(
        egui::pos2(fx + CELL_SIZE / 2.0, fy + CELL_SIZE / 2.0),
        CELL_SIZE / 2.8,
        egui::Color32::from_rgb(
            profile.apple_color[0],
            profile.apple_color[1],
            profile.apple_color[2],
        ),
    );
}

fn draw_dpad_controls(ui: &mut egui::Ui, game: &mut Game) {
    let button_size = egui::vec2(70.0, 70.0);
    let spacing = egui::vec2(8.0, 8.0);
    let arrow_color = egui::Color32::from_gray(190);

    egui::Grid::new("dpad_grid")
        .spacing(spacing)
        .show(ui, |ui| {
            ui.label("");

            let up_btn = ui.add_sized(button_size, egui::Button::new(""));
            if up_btn.clicked() && game.direction != Direction::Down {
                game.direction = Direction::Up;
            }
            draw_arrow_up(&ui.painter_at(up_btn.rect), up_btn.rect, arrow_color);
            
            ui.end_row();

            let left_btn = ui.add_sized(button_size, egui::Button::new(""));
            if left_btn.clicked() && game.direction != Direction::Right {
                game.direction = Direction::Left;
            }
            draw_arrow_left(&ui.painter_at(left_btn.rect), left_btn.rect, arrow_color);

            ui.label("");

            let right_btn = ui.add_sized(button_size, egui::Button::new(""));
            if right_btn.clicked() && game.direction != Direction::Left {
                game.direction = Direction::Right;
            }
            draw_arrow_right(&ui.painter_at(right_btn.rect), right_btn.rect, arrow_color);

            ui.end_row();

            ui.label("");

            let down_btn = ui.add_sized(button_size, egui::Button::new(""));
            if down_btn.clicked() && game.direction != Direction::Up {
                game.direction = Direction::Down;
            }
            draw_arrow_down(&ui.painter_at(down_btn.rect), down_btn.rect, arrow_color);

            ui.end_row();
        });
}

fn draw_arrow_up(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width() * 0.4;

    let points = vec![
        egui::pos2(center.x, center.y - size / 2.0),
        egui::pos2(center.x - size / 2.0, center.y + size / 2.0),
        egui::pos2(center.x + size / 2.0, center.y + size / 2.0),
    ];

    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}

fn draw_arrow_down(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width() * 0.4;

    let points = vec![
        egui::pos2(center.x, center.y + size / 2.0),
        egui::pos2(center.x - size / 2.0, center.y - size / 2.0),
        egui::pos2(center.x + size / 2.0, center.y - size / 2.0),
    ];
    
    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}

fn draw_arrow_left(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width() * 0.4;

    let points = vec![
        egui::pos2(center.x - size / 2.0, center.y),
        egui::pos2(center.x + size / 2.0, center.y - size / 2.0),
        egui::pos2(center.x + size / 2.0, center.y + size / 2.0),
    ];
    
    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}

fn draw_arrow_right(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width() * 0.4;

    let points = vec![
        egui::pos2(center.x + size / 2.0, center.y),
        egui::pos2(center.x - size / 2.0, center.y - size / 2.0),
        egui::pos2(center.x - size / 2.0, center.y + size / 2.0),
    ];
    
    painter.add(egui::Shape::convex_polygon(points, color, egui::Stroke::NONE));
}