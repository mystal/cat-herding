#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::{Cursor, WindowMode, WindowResolution};
// use bevy_kira_audio::AudioPlugin;
// TODO: Move to physics.
use bevy_rapier2d::prelude::*;

// mod app;
// mod config;
// mod entities;
// mod level;
// mod renderer;
// mod world;
// mod sounds;
// mod party;

mod log;
mod window;

const GAME_SIZE: [u32; 2] = [400, 300];
const DEFAULT_SCALE: u8 = 2;
const ALLOW_EXIT: bool = cfg!(not(target_arch = "wasm32"));

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    InGame,
}

fn main() {
    // eprintln!("{}", env!("CARGO_PKG_NAME"));

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
        .insert_resource(ClearColor(Color::rgb_u8(24, 24, 24)))

        // External plugins
        .add_plugins(default_plugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy_egui::EguiPlugin)
        .insert_resource(bevy_egui::EguiSettings {
            // TODO: Take DPI scaling into account as well.
            // scale_factor: (saved_window_state.scale as f64) / 2.0,
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(30.0))

        // App setup
        .add_state::<AppState>()
        .add_plugins((
            window::WindowPlugin::new(saved_window_state),
            // debug::DebugPlugin,
            // game::GamePlugin,
            // physics::PhysicsPlugin,
        ));

    if ALLOW_EXIT {
        app.add_systems(Update, bevy::window::close_on_esc);
    }

    app.run();
}
