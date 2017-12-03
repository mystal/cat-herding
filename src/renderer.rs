use std::rc::Rc;

use cgmath::{self, Matrix4};
use cgmath::prelude::*;
use entities::Camera;
use midgar::{Midgar, Surface};
use midgar::graphics::shape::ShapeRenderer;
use midgar::graphics::sprite::{DrawTexture, MagnifySamplerFilter, Sprite, SpriteDrawParams, SpriteRenderer};
use midgar::graphics::texture::TextureRegion;

use config;
use world::*;

pub struct GameRenderer {
    projection: Matrix4<f32>,
    sprite: SpriteRenderer,
    shape: ShapeRenderer,

    cat_box: TextureRegion,
    basic_cat: TextureRegion,
    wizard_dog: TextureRegion,
}

impl GameRenderer {
    pub fn new(midgar: &Midgar) -> Self {
        // Load textures.
        let cat_box = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/cat_box.png", false));
            TextureRegion::new(texture)
        };
        let basic_cat = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/basic_cat.png", false));
            TextureRegion::with_sub_field(texture, (0, 0), (32, 32))
        };
        let wizard_dog = {
            let texture = Rc::new(midgar.graphics().load_texture("assets/wizard_dog.png", false));
            TextureRegion::new(texture)
        };

        let projection = cgmath::ortho(-(config::SCREEN_SIZE.x as f32 / 2.0), config::SCREEN_SIZE.x as f32 / 2.0,
                                       config::SCREEN_SIZE.y as f32 / 2.0, -(config::SCREEN_SIZE.y as f32 / 2.0),
                                       -1.0, 1.0);

        GameRenderer {
            projection: projection,
            sprite: SpriteRenderer::new(midgar.graphics().display(), projection),
            shape: ShapeRenderer::new(midgar.graphics().display(), projection),

            cat_box,
            basic_cat,
            wizard_dog,
        }
    }

    pub fn render(&mut self, midgar: &Midgar, dt: f32, world: &GameWorld, camera: &Camera) {
        // set the camera view
        let camera_pos = camera.pos.extend(0.0);
        let view = cgmath::Matrix4::look_at(cgmath::Point3::from_vec(camera_pos),
                                            cgmath::Point3::new(0.0, 0.0, -1.0) + camera_pos,
                                            cgmath::vec3(0.0, 1.0, 0.0));

        let combined = self.projection * view;
        self.sprite.set_projection_matrix(combined);
        self.shape.set_projection_matrix(combined);

        // Get framebuffer target.
        let mut target = midgar.graphics().display().draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

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
                         draw_params, &mut target);

        // Draw cats!
        for cat in &world.cats {
            self.sprite.draw(&self.basic_cat.draw(cat.pos.x, cat.pos.y),
                             draw_params, &mut target);
        }

        // Draw dog, woof.
        self.sprite.draw(&self.wizard_dog.draw(world.dog.pos.x, world.dog.pos.y),
                         draw_params, &mut target);

        target.finish().unwrap();
    }
}
