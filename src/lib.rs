pub mod constants;
pub mod database;
pub mod game_logic;
pub mod resources;
pub mod ui;

use bevy::gizmos::GizmoPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::prelude::GilrsPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::PresentMode;
use bevy::winit::WinitSettings;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use android_activity::AndroidApp;
use qrcode::{Color as QrColor, QrCode};

use database::Database;
use game_logic::{load_leaderboard, load_user_data};
use resources::*;
use ui::ui_system;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct DatabaseSetupSet;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppReadyState {
    #[default]
    Initializing,
    Ready,
}

fn check_app_ready(
    time: Res<Time>,
    mut next_state: ResMut<NextState<AppReadyState>>,
) {
    if time.elapsed_seconds() > 0.5 {
        info!("App initialized, setting state to Ready.");
        next_state.set(AppReadyState::Ready);
    }
}

fn configure_egui_fonts(mut contexts: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();
    let font_data =
        egui::FontData::from_static(include_bytes!("../assets/fonts/Poppins-Regular.ttf"));
    fonts.font_data.insert("poppins".to_owned(), font_data);
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "poppins".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "poppins".to_owned());
    contexts.ctx_mut().set_fonts(fonts);
}

fn generate_qr_codes(mut qr_textures: ResMut<QRCodeTextures>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    let release_url = "https://github.com/mintykiera/snake-game/releases/latest";

    let code = QrCode::new(release_url.as_bytes()).unwrap();
    let egui_image = egui::ColorImage {
        size: [code.width(), code.width()],
        pixels: code
            .to_colors()
            .into_iter()
            .map(|c| if c == QrColor::Dark { egui::Color32::BLACK } else { egui::Color32::WHITE })
            .collect(),
    };
    let texture_handle = ctx.load_texture("android_qr", egui_image, Default::default());
    qr_textures.android_qr = Some(texture_handle.id());
    qr_textures.ios_qr = Some(texture_handle.id());
}

// FIXED: handle Option and provide a safe fallback
fn setup_database(mut commands: Commands, app: NonSend<AndroidApp>) {
    let data_dir: std::path::PathBuf = app
        .internal_data_path()
        .or_else(|| app.external_data_path())
        .unwrap_or_else(|| std::path::PathBuf::from("./snake_game_data"));

    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        error!("Failed to create data directory: {}", e);
    }

    let db = Database::new(data_dir.to_string_lossy().to_string());
    commands.insert_resource(db);
}

fn spawn_demo(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.2, 0.8, 0.3),
            custom_size: Some(Vec2::new(300.0, 300.0)),
            ..default()
        },
        ..default()
    });
}

fn egui_debug_overlay(mut ctxs: EguiContexts) {
    egui::Area::new("dbg_overlay".into())
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(ctxs.ctx_mut(), |ui| {
            ui.label("Hello from Bevy on Android (GL) 👋");
        });
}

#[bevy_main]
fn main() {
    std::env::set_var("WGPU_BACKEND", "gl");

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<PbrPlugin>()
                .disable::<GilrsPlugin>()
                .disable::<GizmoPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snake Game".to_string(),
                        resolution: (400., 800.).into(),
                        resizable: false,
                        present_mode: PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::GL),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .insert_resource(WinitSettings::game())
        .add_plugins(EguiPlugin)
        .init_resource::<GameState>()
        .init_resource::<Game>()
        .init_resource::<UserProfile>()
        .init_resource::<Leaderboard>()
        .init_resource::<EguiInitialized>()
        .init_resource::<QRCodeTextures>()
        .init_state::<AppReadyState>()
        .add_systems(
            Update,
            check_app_ready.run_if(in_state(AppReadyState::Initializing)),
        )
        .add_systems(
            Startup,
            (
                setup_database.in_set(DatabaseSetupSet),
                (load_user_data, load_leaderboard).after(DatabaseSetupSet),
            ),
        )
        .add_systems(
            OnEnter(AppReadyState::Ready),
            (
                spawn_demo,
                configure_egui_fonts,
                generate_qr_codes,
            ),
        )
        .add_systems(
            Update,
            (egui_debug_overlay, ui_system).run_if(in_state(AppReadyState::Ready)),
        )
        .run();
}