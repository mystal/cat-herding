#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run},
};

mod app;
mod config;
mod entities;
mod level;
mod renderer;
mod world;
mod sounds;
mod party;

fn main() {
    #[cfg(target_arch = "wasm32")]
    ::std::panic::set_hook(Box::new(|info| {
        console!(log, info.to_string());
    }));

    run::<app::GameApp>(
        "Cat Chaser",
        Vector::new(config::SCREEN_SIZE.x, config::SCREEN_SIZE.y),
        Settings::default());
}
