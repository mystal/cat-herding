#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::{Cursor, WindowMode, WindowResolution};
use bevy_kira_audio::AudioPlugin;

mod assets;
mod cats;
mod debug;
mod dog;
mod game;
mod input;
mod level;
mod log;
mod physics;
mod window;

const GAME_SIZE: UVec2 = UVec2::new(400, 300);
const DEFAULT_SCALE: u8 = 2;
const ALLOW_EXIT: bool = cfg!(not(target_arch = "wasm32"));

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    StartMenu,
    Credits,
    HowToPlay,
    Playing,
    Won,
    GameOver,
}

fn main() {
    // When building for WASM, print panics to the browser console.
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // TODO: Try to initialize logging before this. Maybe we can also make this code run in a plugin.
    let saved_window_state = window::load_window_state();
    let cursor = Cursor {
        visible: true,
        ..default()
    };

    // Configure DefaultPlugins.
    let default_plugins = DefaultPlugins
        .set(log::log_plugin())
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: window::WINDOW_TITLE.into(),
                position: saved_window_state.position,
                resolution: WindowResolution::new(
                    GAME_SIZE[0] as f32 * saved_window_state.scale as f32,
                    GAME_SIZE[1] as f32 * saved_window_state.scale as f32,
                ),
                resizable: false,
                mode: WindowMode::Windowed,
                cursor,
                ..default()
            }),
            ..default()
        });

    let mut app = App::new();
    app
        .insert_resource(ClearColor(Color::BLACK))

        // External plugins
        .add_plugins((
            default_plugins,
            FrameTimeDiagnosticsPlugin::default(),
            bevy_egui::EguiPlugin,
            AudioPlugin,
        ))

        // App setup
        .init_state::<AppState>()
        .add_plugins((
            window::WindowPlugin::new(saved_window_state),
            input::InputPlugin,
            physics::PhysicsPlugin,
            assets::AssetsPlugin,
            debug::DebugPlugin,
            game::GamePlugin,
            level::LevelPlugin,
            cats::CatsPlugin,
        ));

    if ALLOW_EXIT {
        app.add_systems(Update, bevy::window::close_on_esc);
    }

    app.run();
}
