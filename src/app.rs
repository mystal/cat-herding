use midgar::{self, KeyCode, Midgar};

use config;
use cgmath;
use entities::Camera;
use world::GameWorld;
use renderer::GameRenderer;

pub struct GameApp<'a> {
    camera: Camera,
    world: GameWorld,
    renderer: GameRenderer<'a>,
}

impl<'a> midgar::App for GameApp<'a> {
    fn create(midgar: &Midgar) -> Self {
        GameApp {
            world: GameWorld::new(),
            camera: Camera {
                pos: cgmath::vec2(config::GAME_SIZE.x as f32 / 2.0, config::GAME_SIZE.y as f32 / 2.0),
                bounds: config::GAME_SIZE.cast::<f32>(),
                zoom: 1,
            },
            renderer: GameRenderer::new(midgar),
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        let dt = midgar.time().delta_time() as f32;

        self.world.update(midgar, dt);

        self.renderer.render(midgar, dt, &self.world, &self.camera);
    }
}
