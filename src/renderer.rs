use std::rc::Rc;

use cgmath::{self, Matrix4, Vector3};
use cgmath::prelude::*;
use entities::Camera;
use midgar::{Midgar, Surface};
use midgar::graphics::animation::{Animation, PlayMode};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, Sprite, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::TextureRegion;

use config;
use entities::Facing;
use world::*;

pub struct GameRenderer {
    projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,

    cat_box: TextureRegion,
    basic_cat_walk: TextureRegion,
    basic_cat_walk_alt: TextureRegion,
    basic_cat_walk_animation: Animation,
    basic_cat_walk_time: f32,
    wizard_dog_idle_animation: Animation,
    wizard_dog_idle_time: f32,
    wizard_dog_run_animation: Animation,
    // TODO: Move this to Dog to start the animation at the right time.
    wizard_dog_run_time: f32,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        // Load textures.
        let cat_box = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/cat_box.png", false));
            TextureRegion::new(texture)
        };
        let (basic_cat_walk, basic_cat_walk_alt) = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/basic_cat_walk.png", false));
            (TextureRegion::with_sub_field(texture.clone(), (0, 0), (32, 32)),
             TextureRegion::with_sub_field(texture.clone(), (32, 0), (32, 32)))
        };
        let mut basic_cat_walk_animation = Animation::new(0.2, &[basic_cat_walk.clone(), basic_cat_walk_alt.clone()])
            .unwrap();
        basic_cat_walk_animation.play_mode = PlayMode::Loop;
        let wizard_dog_idle = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/wizard_dog_idle.png", false));
            TextureRegion::split(texture, (32, 32))
        };
        let mut wizard_dog_idle_animation = Animation::new(0.2, &wizard_dog_idle)
            .unwrap();
        wizard_dog_idle_animation.play_mode = PlayMode::Loop;
        let wizard_dog_run = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/wizard_dog_run.png", false));
            TextureRegion::split(texture, (32, 32))
        };
        let mut wizard_dog_run_animation = Animation::new(0.1, &wizard_dog_run)
            .unwrap();
        wizard_dog_run_animation.play_mode = PlayMode::Loop;

        let projection = cgmath::ortho(-(config::GAME_SIZE.x as f32 / 2.0), config::GAME_SIZE.x as f32 / 2.0,
                                       config::GAME_SIZE.y as f32 / 2.0, -(config::GAME_SIZE.y as f32 / 2.0),
                                       -1.0, 1.0);

        GameRenderer {
            projection: projection,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),

            cat_box,
            basic_cat_walk,
            basic_cat_walk_alt,
            basic_cat_walk_animation,
            basic_cat_walk_time: 0.0,
            wizard_dog_idle_animation,
            wizard_dog_idle_time: 0.0,
            wizard_dog_run_animation,
            wizard_dog_run_time: 0.0,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld, camera: &Camera) {
        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        match world.game_state {
            GameState::StartMenu => {
                //do a thing
            },
            GameState::HowToPlay => {
                //do a different thing
            },
            GameState::Running | GameState::Won | GameState::GameOver => {
                self.draw_world(midgar, dt, world, camera, &mut target);
            },
        }

        target.finish().unwrap();
    }

    fn draw_world<S: Surface>(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld, camera: &Camera, target: &mut S) {
        // set the camera view
        let camera_pos = camera.pos.extend(0.0);
        let view = cgmath::Matrix4::look_at(cgmath::Point3::from_vec(camera_pos),
                                            cgmath::Point3::new(0.0, 0.0, -1.0) + camera_pos,
                                            cgmath::vec3(0.0, 1.0, 0.0));

        let combined = self.projection * view;
        self.sprite.set_projection_matrix(combined);
        self.shape.set_projection_matrix(combined);

        // Some colors!
        let white = [1.0, 1.0, 1.0];
        let grey = [0.5, 0.5, 0.5];
        let black = [0.0, 0.0, 0.0];
        let blue_violet = [138.0 / 255.0, 43.0 / 255.0, 226.0 / 255.0];

        let draw_params = SpriteDrawParams::new()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .alpha(true);

        // Draw cat box.
        self.sprite.draw(&self.cat_box.draw(world.cat_box().pos.x, world.cat_box().pos.y),
                         draw_params, target);

        // Draw cats!
        self.basic_cat_walk_time += dt;
        for cat in &world.cats {
            let mut sprite = self.basic_cat_walk_animation.current_key_frame(self.basic_cat_walk_time)
                .draw(cat.pos.x, cat.pos.y);
            sprite.set_flip_x(cat.facing == Facing::Right);
            sprite.set_color(cgmath::Vector3::new(1.0, 1.0 - cat.normalized_jitter(), 1.0 - cat.normalized_jitter()));
            self.sprite.draw(&sprite, draw_params, target);
        }

        // Draw dog, woof.
        self.wizard_dog_idle_time += dt;
        self.wizard_dog_run_time += dt;
        let mut sprite = if world.dog.vel.is_zero() {
            self.wizard_dog_idle_animation.current_key_frame(self.wizard_dog_idle_time)
                .draw(world.dog.pos.x, world.dog.pos.y)
        } else {
            self.wizard_dog_run_animation.current_key_frame(self.wizard_dog_run_time)
                .draw(world.dog.pos.x, world.dog.pos.y)
        };
        sprite.set_flip_x(world.dog.facing == Facing::Right);
        self.sprite.draw(&sprite, draw_params, target);

        match world.game_state {
            GameState::Running => {
                //ui stuff
            },
            GameState::Won => {
                //ui stuff
            },
            GameState::GameOver => {
                //ui stuff
            },
            _ => {},
        }
    }
}
