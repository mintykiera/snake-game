pub mod constants;
pub mod game_logic;
pub mod resources;
pub mod ui;

use std::sync::mpsc::{Receiver, Sender};
use eframe::egui;
use resources::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub enum AsyncMessage {
    ProfileLoaded(UserProfile),
    LeaderboardLoaded(Leaderboard),
    ScoreSubmitted,
}

pub enum AsyncCommand {
    LoadProfile,
    LoadLeaderboard,
    SubmitScore(u32, UserProfile),
    UpdateProfile(UserProfile),
}

#[derive(Serialize, Deserialize)]
struct SaveState {
    profile: UserProfile,
}

pub struct SnakeApp {
    game: Game,
    state: GameState,
    profile: UserProfile,
    leaderboard: Leaderboard,
    qr_textures: QRCodeTextures,
    rx: Receiver<AsyncMessage>,
    tx: Sender<AsyncCommand>,
}

impl SnakeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let (tx_to_ui, rx_from_async) = std::sync::mpsc::channel();
        let (tx_to_async, rx_from_ui) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async_loop(rx_from_ui, tx_to_ui));
        });

        let profile = if let Some(storage) = cc.storage {
            eframe::get_value::<SaveState>(storage, eframe::APP_KEY)
                .map(|s| s.profile)
                .unwrap_or_default()
        } else {
            UserProfile::default()
        };

        let _ = tx_to_async.send(AsyncCommand::LoadLeaderboard);

        Self {
            game: Game::default(),
            state: GameState::default(),
            profile,
            leaderboard: Leaderboard::default(),
            qr_textures: QRCodeTextures::default(),
            rx: rx_from_async,
            tx: tx_to_async,
        }
    }
}

impl eframe::App for SnakeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let save_data = SaveState {
            profile: self.profile.clone(),
        };
        eframe::set_value(storage, eframe::APP_KEY, &save_data);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            match self.state.current_screen {
                Screen::MainMenu => {
                    std::process::exit(0);
                }
                Screen::Playing => {
                    self.state.current_screen = Screen::MainMenu;
                    self.game = Game::default();
                }
                _ => {
                    self.state.current_screen = Screen::MainMenu;
                }
                Screen::Leaderboard => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let should_refresh = ui::leaderboard::show_leaderboard_screen(ui, &mut self.state, &self.leaderboard);
                        if should_refresh {
                            let _ = self.tx.send(AsyncCommand::LoadLeaderboard);
                        }
                    });
                }
            }
        }

        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AsyncMessage::ProfileLoaded(p) => self.profile = p,
                AsyncMessage::LeaderboardLoaded(l) => self.leaderboard = l,
                AsyncMessage::ScoreSubmitted => {
                    let _ = self.tx.send(AsyncCommand::LoadLeaderboard);
                }
            }
        }

        if self.state.current_screen == Screen::Playing {
            game_logic::handle_input(ctx, &mut self.game);
            let dt = ctx.input(|i| i.stable_dt);
            let should_submit = game_logic::update_game(dt, &mut self.game, &mut self.profile);

            if should_submit {
                 let _ = self.tx.send(AsyncCommand::SubmitScore(self.game.score, self.profile.clone()));
                 ctx.request_repaint(); 
            }
            ctx.request_repaint(); 
        }

        match self.state.current_screen {
            Screen::Playing => {
                egui::CentralPanel::default().show(ctx, |ui| {
                      ui::game_screen::show_game_screen(ui, &mut self.state, &mut self.game, &self.profile);
                });
            }
            Screen::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui::main_menu::show_main_menu(ui, &mut self.state, &self.profile);
                });
            }
            Screen::Leaderboard => {
                 egui::CentralPanel::default().show(ctx, |ui| {
                    ui::leaderboard::show_leaderboard_screen(ui, &mut self.state, &self.leaderboard);
                });
            }
            Screen::Settings => {
                 egui::CentralPanel::default().show(ctx, |ui| {
                    ui::settings::show_settings_screen(ui, &mut self.state);
                });
            }
            Screen::Profile => {
                 egui::CentralPanel::default().show(ctx, |ui| {
                    let should_sync = ui::profile::show_profile_screen(ui, &mut self.state, &mut self.profile);
                    if should_sync {
                        let _ = self.tx.send(AsyncCommand::UpdateProfile(self.profile.clone()));
                        ctx.request_repaint();
                    }
                });
            }
             Screen::Share => {
                 egui::CentralPanel::default().show(ctx, |ui| {
                    ui::share::show_share_screen(ui, &mut self.state, &mut self.qr_textures, ctx);
                });
            }
        }
    }
}

async fn async_loop(rx: Receiver<AsyncCommand>, tx: Sender<AsyncMessage>) {
    let base_url = option_env!("FIREBASE_URL").unwrap_or("ENV_NOT_FOUND");

    let client = reqwest::Client::new();

    while let Ok(cmd) = rx.recv() {
        match cmd {
            AsyncCommand::LoadProfile => { }
            AsyncCommand::LoadLeaderboard => {
                let url = format!("{}leaderboard.json", base_url);
                
                if let Ok(resp) = client.get(&url).send().await {
                    if let Ok(text) = resp.text().await {
                        if text == "null" {
                            let _ = tx.send(AsyncMessage::LeaderboardLoaded(Leaderboard { entries: vec![] }));
                        } else if let Ok(map) = serde_json::from_str::<HashMap<String, LeaderboardEntry>>(&text) {
                            let mut entries: Vec<LeaderboardEntry> = map.into_values().collect();
                            entries.sort_by(|a, b| b.score.cmp(&a.score));
                            entries.truncate(10);
                            let _ = tx.send(AsyncMessage::LeaderboardLoaded(Leaderboard { entries }));
                        }
                    }
                }
            }
            AsyncCommand::UpdateProfile(profile) => {
                 let url = format!("{}leaderboard/{}.json", base_url, profile.user_id);
                 let entry = LeaderboardEntry {
                    user_id: profile.user_id.clone(),
                    username: profile.username.clone(),
                    score: profile.high_score,
                 };

                 let _ = client.patch(&url).json(&entry).send().await;
                 let _ = tx.send(AsyncMessage::ScoreSubmitted);
            }
            AsyncCommand::SubmitScore(score, profile) => {
                 let url = format!("{}leaderboard/{}.json", base_url, profile.user_id);
                 let entry = LeaderboardEntry {
                    user_id: profile.user_id.clone(),
                    username: profile.username.clone(),
                    score,
                 };
                 let _ = client.patch(&url).json(&entry).send().await;
                 let _ = tx.send(AsyncMessage::ScoreSubmitted);
            }
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}

#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use eframe::NativeOptions;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let mut options = NativeOptions::default();

    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    eframe::run_native(
        "Snake Game",
        options,
        Box::new(|cc| Ok(Box::new(SnakeApp::new(cc)))),
    ).unwrap();
}