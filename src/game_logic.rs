use bevy::prelude::{KeyCode, Res, ResMut, Time};
use bevy::input::Input;
use rand::Rng;

use crate::resources::*;
use crate::constants::*;
use crate::database::Database;

pub fn save_user_data(profile: &UserProfile, db: &mut Database) {
    let key = "user_profile";
    if let Ok(value_json) = serde_json::to_string(profile) {
        db.save_data(key, &value_json);
    }
}

pub fn load_user_data(
    mut profile: ResMut<UserProfile>,
    mut db_res: ResMut<Database>
) {
    let key = "user_profile";
    if let Some(data_str) = db_res.load_data(key) {
        if let Ok(loaded_profile) = serde_json::from_str(&data_str) {
            *profile = loaded_profile;
            println!("Successfully loaded user profile.");
            return;
        }
    }
    println!("No profile found. Creating a new one.");
    if profile.username.is_empty() {
        profile.username = format!("Player_{}", &profile.user_id[..6]);
    }
    save_user_data(&profile, &mut *db_res);
}

pub fn save_leaderboard(leaderboard: &Leaderboard, db: &mut Database) {
    let key = "leaderboard";
    if let Ok(value_json) = serde_json::to_string(leaderboard) {
        db.save_data(key, &value_json);
    }
}

pub fn load_leaderboard(
    mut leaderboard: ResMut<Leaderboard>,
    mut db_res: ResMut<Database>
) {
    let key = "leaderboard";
    if let Some(data_str) = db_res.load_data(key) {
        if let Ok(loaded_board) = serde_json::from_str(&data_str) {
            *leaderboard = loaded_board;
            println!("Successfully loaded leaderboard.");
            return;
        }
    }
    println!("No leaderboard found. Creating a new one.");
    save_leaderboard(&leaderboard, &mut *db_res);
}

pub fn handle_keyboard_input(keyboard: &Res<Input<KeyCode>>, game: &mut Game) {
    if keyboard.just_pressed(KeyCode::Up) || keyboard.just_pressed(KeyCode::W) {
        if game.direction != Direction::Down { game.direction = Direction::Up; }
    } else if keyboard.just_pressed(KeyCode::Down) || keyboard.just_pressed(KeyCode::S) {
        if game.direction != Direction::Up { game.direction = Direction::Down; }
    } else if keyboard.just_pressed(KeyCode::Left) || keyboard.just_pressed(KeyCode::A) {
        if game.direction != Direction::Right { game.direction = Direction::Left; }
    } else if keyboard.just_pressed(KeyCode::Right) || keyboard.just_pressed(KeyCode::D) {
        if game.direction != Direction::Left { game.direction = Direction::Right; }
    } else if keyboard.just_pressed(KeyCode::Space) {
        game.paused = !game.paused;
    }
}

pub fn run_game_logic(
    time: &Time,
    game: &mut Game,
    profile: &mut UserProfile,
    leaderboard: &mut Leaderboard,
    db: &mut Database
) {
    if game.game_over || game.paused { return; }
    game.timer += time.delta_seconds();
    if game.timer < 0.15 { return; }
    game.timer = 0.0;

    let head = game.snake[0];
    let mut new_head = match game.direction {
        Direction::Up => (head.0, head.1 - 1),
        Direction::Down => (head.0, head.1 + 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
    };

    if new_head.0 < 0 { new_head.0 = GRID_SIZE - 1; }
    else if new_head.0 >= GRID_SIZE { new_head.0 = 0; }
    if new_head.1 < 0 { new_head.1 = GRID_SIZE - 1; }
    else if new_head.1 >= GRID_SIZE { new_head.1 = 0; }
    
    if game.snake.contains(&new_head) {
        game.game_over = true;
        
        if game.score > profile.high_score {
            profile.high_score = game.score;
            save_user_data(profile, db);
        }

        let player_id = &profile.user_id;

        let existing_entry = leaderboard.entries.iter_mut()
            .find(|entry| &entry.user_id == player_id);

        if let Some(entry) = existing_entry {
            if game.score > entry.score {
                entry.score = game.score;
                entry.username = profile.username.clone();
            }
        } else {
            leaderboard.entries.push(LeaderboardEntry {
                user_id: player_id.clone(),
                username: profile.username.clone(),
                score: game.score,
            });
        }
        
        leaderboard.entries.sort_by(|a, b| b.score.cmp(&a.score));
        leaderboard.entries.truncate(10);
        save_leaderboard(leaderboard, db);
        
        return;
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
}