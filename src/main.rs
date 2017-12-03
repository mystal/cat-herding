extern crate cgmath;
extern crate midgar;
extern crate rand;

mod app;
mod config;
mod entities;
mod level;
mod renderer;
mod world;

fn main() {
    let app_config = midgar::MidgarAppConfig::new()
        .with_title("Cat Herding")
        .with_screen_size((config::SCREEN_SIZE.x, config::SCREEN_SIZE.y));
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(app_config);
    app.run();
}
