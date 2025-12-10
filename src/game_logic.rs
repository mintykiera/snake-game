use crate::constants::*;
use crate::resources::{Direction, Game, UserProfile};
use rand::Rng;
use eframe::egui;

pub fn handle_input(ctx: &egui::Context, game: &mut Game) {
    if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W)) {
        if game.direction != Direction::Down {
            game.direction = Direction::Up;
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S)) {
        if game.direction != Direction::Up {
            game.direction = Direction::Down;
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A)) {
        if game.direction != Direction::Right {
            game.direction = Direction::Left;
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D)) {
        if game.direction != Direction::Left {
            game.direction = Direction::Right;
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
        game.paused = !game.paused;
    }
}

pub fn update_game(dt: f32, game: &mut Game, profile: &mut UserProfile) -> bool {
    if game.game_over || game.paused {
        return false;
    }

    game.timer += dt;
    if game.timer < 0.15 {
        return false;
    }
    game.timer = 0.0;

    let head = game.snake[0];
    let mut new_head = match game.direction {
        Direction::Up => (head.0, head.1 - 1),
        Direction::Down => (head.0, head.1 + 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
    };

    if new_head.0 < 0 {
        new_head.0 = GRID_SIZE - 1;
    } else if new_head.0 >= GRID_SIZE {
        new_head.0 = 0;
    }
    if new_head.1 < 0 {
        new_head.1 = GRID_SIZE - 1;
    } else if new_head.1 >= GRID_SIZE {
        new_head.1 = 0;
    }

    if game.snake.contains(&new_head) {
        game.game_over = true;
        if game.score > profile.high_score {
            profile.high_score = game.score;
        }
        return true;
    }

    game.snake.insert(0, new_head);

    if new_head == game.food {
        game.score += 1;
        let mut rng = rand::thread_rng();
        loop {
            let food = (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE));
            if !game.snake.contains(&food) {
                game.food = food;
                break;
            }
        }
    } else {
        game.snake.pop();
    }
    
    false
} 