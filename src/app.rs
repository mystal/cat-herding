use std::time::Instant;

use quicksilver::{
    Result,
    input::{ButtonState, Key},
    lifecycle::{Event, State, Window},
};

use crate::config;
use crate::entities::Camera;
use crate::world::GameWorld;
use crate::renderer::GameRenderer;

pub struct GameApp {
    camera: Camera,
    world: GameWorld,
    renderer: GameRenderer,
    //last_update: Instant,
}

impl State for GameApp {
    fn new() -> Result<Self> {
        let world = GameWorld::new();
        //world.sounds.intro_music.set_volume(0.2);
        //world.sounds.intro_music.play();

        Ok(Self {
            world,
            camera: Camera {
                pos: cgmath::vec2(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0),
                bounds: config::GAME_SIZE.cast::<f32>(),
                zoom: 1,
            },
            renderer: GameRenderer::new(),
            //last_update: Instant::now(),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        //dbg!(event);

        match event {
            Event::Key(Key::Escape, ButtonState::Pressed) => window.close(),
            _ => {}
        }

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // TODO: Get real delta time. Can we do that with WASM?
        let dt = 1.0 / 60.0;

        //if !self.sounds.intro_music.is_playing() && !self.sounds.background_music.is_playing() {
        //    self.sounds.background_music.set_volume(0.2);
        //    self.sounds.background_music.play();
        //}
        self.world.update(window, dt);
        self.renderer.tick_animations(0.0)?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        //let dt = self.last_update.elapsed();
        //let dt = dt.as_secs() as f32 + (dt.subsec_nanos() as f32 / 1_000_000_000.0);

        //dbg!(dt);

        self.renderer.render(window, &self.world, &self.camera)?;

        //self.last_update = Instant::now();

        Ok(())
    }
}
