use cgmath::{self, Matrix4};
use cgmath::prelude::*;
use crate::entities::{Camera, DogState, CatState, CatType};
use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Rectangle, Shape, Transform},
    graphics::{Animation, Background, Color, Font, FontStyle, Image, View},
    lifecycle::{Asset, Window},
};

use crate::config;
use crate::entities::{CAT_COLORS, Facing};
use crate::party::PartyItemKind;
use crate::world::*;

pub struct GameRenderer {
    start_menu: Asset<Image>,
    how_to_play: Asset<Image>,

    background: Asset<Image>,
    cat_box: Asset<Image>,
    basic_cat_walk: Asset<Animation>,
    basic_cat_idle: Asset<Animation>,
    basic_cat_ball: Asset<Animation>,
    fat_cat_idle: Asset<Animation>,
    fat_cat_walk: Asset<Animation>,
    fat_cat_ball: Asset<Animation>,
    kitten_idle: Asset<Animation>,
    kitten_walk: Asset<Animation>,
    wizard_dog_idle: Asset<Animation>,
    wizard_dog_run: Asset<Animation>,
    // TODO: Move this to Dog to start the animation at the right time.

    linda_cat: Asset<Animation>,
    morgan_kitten: Asset<Animation>,
    justin_spin: Asset<Animation>,
    gabe_dog: Asset<Animation>,
    guest_fox: Asset<Animation>,

    font: Asset<Font>,
    //enter_to_play_text: Image,
    cat_face: Asset<Image>,

    game_time: f32,
}

impl GameRenderer {
    pub fn new() -> Self {

        // Load textures.
        let start_menu = Asset::new(Image::load("start_menu_background.png"));
        let how_to_play = Asset::new(Image::load("how_to_play.png"));
        let background = Asset::new(Image::load("hardwood_floor.png").map(|image| {
            let rect = Rectangle::new((0, 0), (config::SCREEN_SIZE.x, config::SCREEN_SIZE.y));
            image.subimage(rect)
        }));
        let cat_box = Asset::new(Image::load("cat_box.png"));

        let basic_cat_walk = Asset::new(Image::load("walk/basic_cat_walk.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let basic_cat_idle = Asset::new(Image::load("idle/basic_cat_idle.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
                Rectangle::new((64, 0), (32, 32)),
                Rectangle::new((96, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let basic_cat_ball = Asset::new(Image::load("ball/basic_cat_bowling_ball.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
                Rectangle::new((64, 0), (32, 32)),
                Rectangle::new((96, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let fat_cat_walk = Asset::new(Image::load("walk/fat_cat_walk.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let fat_cat_idle = Asset::new(Image::load("idle/fat_cat_idle.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let fat_cat_ball = Asset::new(Image::load("ball/fat_cat_bowling_ball.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let kitten_walk = Asset::new(Image::load("walk/kitten_walk.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let kitten_idle = Asset::new(Image::load("idle/kitten_idle.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let wizard_dog_idle = Asset::new(Image::load("idle/wizard_dog_idle.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let wizard_dog_run = Asset::new(Image::load("walk/wizard_dog_run.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
                Rectangle::new((64, 0), (32, 32)),
                Rectangle::new((96, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let linda_cat = Asset::new(Image::load("credits/linda_cat.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let morgan_kitten = Asset::new(Image::load("credits/morgan_kitten.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let justin_spin = Asset::new(Image::load("credits/justin_spin.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
                Rectangle::new((64, 0), (32, 32)),
                Rectangle::new((96, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 6)
        }));

        let gabe_dog = Asset::new(Image::load("credits/gabe_dog.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (32, 32)),
                Rectangle::new((32, 0), (32, 32)),
                Rectangle::new((64, 0), (32, 32)),
                Rectangle::new((96, 0), (32, 32)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 6)
        }));

        let guest_fox = Asset::new(Image::load("credits/guest_fox.png").map(|image| {
            let regions = [
                Rectangle::new((0, 0), (20, 20)),
                Rectangle::new((20, 0), (20, 20)),
            ];
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 12)
        }));

        let font = Asset::new(
            Font::load("fonts/Kenney Pixel.ttf")
            //.and_then(|font| {
            //    let style = FontStyle::new(40.0, Color::BLACK);
            //    result(font.render("Sample Text", &style))
            //})
        );

        let cat_face = Asset::new(Image::load("cat_face.png"));

        GameRenderer {
            start_menu,
            how_to_play,

            background,
            cat_box,
            basic_cat_walk,
            basic_cat_idle,
            fat_cat_idle,
            kitten_idle,
            wizard_dog_idle,
            wizard_dog_run,
            kitten_walk,
            fat_cat_walk,
            basic_cat_ball,
            fat_cat_ball,

            linda_cat,
            morgan_kitten,
            justin_spin,
            gabe_dog,
            guest_fox,

            font,
            //enter_to_play_text,
            cat_face: cat_face,

            game_time: 0.0,
        }
    }

    pub fn render(&mut self, window: &mut Window, dt: f32, world: &GameWorld, camera: &Camera) {
        self.game_time += dt;

        while self.game_time >= (1.0 / 60.0) {
            self.game_time -= 1.0 / 60.0;

            // Tick all the animations!
            self.basic_cat_walk.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick basic_cat_walk");
            self.basic_cat_idle.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick basic_cat_idle");
            self.basic_cat_ball.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick basic_cat_ball");
            self.fat_cat_idle.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick fat_cat_idle");
            self.fat_cat_walk.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick fat_cat_walk");
            self.fat_cat_ball.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick fat_cat_ball");
            self.kitten_idle.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick kitten_idle");
            self.kitten_walk.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick kitten_walk");
            self.wizard_dog_idle.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick wizard_dog_idle");
            self.wizard_dog_run.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick wizard_dog_run");

            self.linda_cat.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick linda_cat");
            self.morgan_kitten.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick morgan_kitten");
            self.justin_spin.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick justin_spin");
            self.gabe_dog.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick gabe_dog");
            self.guest_fox.execute(|anim| {
                anim.tick();
                Ok(())
            }).expect("Failed to tick guest_fox");
        }

        match world.game_state {
            GameState::StartMenu => {
                // Draw start menu splash screen!
                self.start_menu.execute(|image| {
                    window.draw(&image.area().with_center((config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0)),
                                Background::Img(image));
                    Ok(())
                });

                // Draw blinking text!
                //if self.game_time.fract() < 0.5 {
                //    window.draw();
                //    self.text.draw_text("Press Enter to play!", &self.font, [0.0, 0.0, 0.0],
                //                        40, 452.0, 542.0, 500, &projection, &mut target);
                //    self.text.draw_text("Press Enter to play!", &self.font, [1.0, 1.0, 1.0],
                //                        40, 450.0, 540.0, 500, &projection, &mut target);
                //}
            },
            GameState::Credits => {
                window.clear(Color::WHITE);

                // Draw our sprites!
                //let mut sprite = self.linda_cat.current_key_frame(self.game_time)
                //    .draw(200.0, 50.0);
                //sprite.set_scale(cgmath::vec2(4.0, 4.0));
                //sprite.set_flip_x(true);
                //self.sprite.draw(&sprite, draw_params, &mut target);

                //let mut sprite = self.morgan_kitten.current_key_frame(self.game_time)
                //    .draw(200.0, 160.0);
                //sprite.set_scale(cgmath::vec2(4.0, 4.0));
                //sprite.set_flip_x(true);
                //self.sprite.draw(&sprite, draw_params, &mut target);

                //let mut sprite = self.justin_spin.current_key_frame(self.game_time)
                //    .draw(200.0, 270.0);
                //sprite.set_scale(cgmath::vec2(4.0, 4.0));
                //sprite.set_flip_x(true);
                //self.sprite.draw(&sprite, draw_params, &mut target);

                //let mut sprite = self.gabe_dog.current_key_frame(self.game_time)
                //    .draw(200.0, 380.0);
                //sprite.set_scale(cgmath::vec2(3.5, 3.5));
                //sprite.set_flip_x(true);
                //self.sprite.draw(&sprite, draw_params, &mut target);

                //let mut sprite = self.guest_fox.current_key_frame(self.game_time)
                //    .draw(200.0, 490.0);
                //sprite.set_scale(cgmath::vec2(3.0, 3.0));
                //self.sprite.draw(&sprite, draw_params, &mut target);

                //// TODO: Draw our names!
                //self.text.draw_text("Linda Cai", &self.font, [0.0, 0.0, 0.0],
                //                    40, 300.0, 60.0, 500, &projection, &mut target);
                //self.text.draw_text("Morgan Tenney", &self.font, [0.0, 0.0, 0.0],
                //                    40, 300.0, 180.0, 500, &projection, &mut target);
                //self.text.draw_text("Justin Hamilton", &self.font, [0.0, 0.0, 0.0],
                //                    40, 300.0, 265.0, 500, &projection, &mut target);
                //self.text.draw_text("Gabriel Martinez", &self.font, [0.0, 0.0, 0.0],
                //                    40, 300.0, 375.0, 500, &projection, &mut target);
                //self.text.draw_text("Thaminda Edirisooriya", &self.font, [0.0, 0.0, 0.0],
                //                    30, 300.0, 485.0, 500, &projection, &mut target);

                //// Draw blinking text!
                //if self.game_time.fract() < 0.5 {
                //    self.text.draw_text("Press Tab to return!", &self.font, [0.0, 0.0, 0.0],
                //                        40, 452.0, 542.0, 500, &projection, &mut target);
                //    self.text.draw_text("Press Tab to return!", &self.font, [1.0, 1.0, 1.0],
                //                        40, 450.0, 540.0, 500, &projection, &mut target);
                //}
            },
            GameState::HowToPlay => {
                window.set_view(View::new(Rectangle::new_sized((800, 600))));

                // Draw how to play splash screen!
                self.how_to_play.execute(|image| {
                    window.draw(&image.area().with_center((config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0)),
                                Background::Img(image));
                    Ok(())
                });

                // Draw corgi idle animation
                self.wizard_dog_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((4.0, 4.0));
                    window.draw_ex(&image.area().with_center((670.0, 50.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                });

                // Draw cat animations
                self.basic_cat_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((3.0, 3.0));
                    window.draw_ex(&image.area().with_center((380.0, 340.0)),
                                   Background::Blended(image, CAT_COLORS[0]), trans, 0.0);
                    Ok(())
                });

                self.fat_cat_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((3.0, 3.0));
                    window.draw_ex(&image.area().with_center((480.0, 340.0)),
                                   Background::Blended(image, CAT_COLORS[3]), trans, 0.0);
                    Ok(())
                });

                self.kitten_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((3.0, 3.0));
                    window.draw_ex(&image.area().with_center((580.0, 340.0)),
                                   Background::Blended(image, CAT_COLORS[2]), trans, 0.0);
                    Ok(())
                });

                // Draw blinking text!
                //if self.game_time.fract() < 0.5 {
                //    self.text.draw_text("Press Enter to play!", &self.font, [0.0, 0.0, 0.0],
                //                        40, 452.0, 542.0, 500, &projection, &mut target);
                //    self.text.draw_text("Press Enter to play!", &self.font, [1.0, 1.0, 1.0],
                //                        40, 450.0, 540.0, 500, &projection, &mut target);
                //}

                window.flush();
            },
            GameState::Running | GameState::Won => {
                self.draw_world(dt, world, camera, window);
                self.draw_ui(dt, world, window);
            },
            GameState::GameOver => {
                self.draw_world(dt, world, camera, window);

                //let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
                //                               config::SCREEN_SIZE.y as f32, 0.0,
                //                               -1.0, 1.0);
                //self.sprite.set_projection_matrix(projection);

                //// Draw the party!
                //for item in &world.the_party.party_items {
                //    let mut sprite = match item.kind {
                //        PartyItemKind::BasicCat => self.basic_cat_idle_animation.current_key_frame(self.game_time),
                //        PartyItemKind::FatCat => self.fat_cat_idle_animation.current_key_frame(self.game_time),
                //        PartyItemKind::Kitten => self.kitten_idle_animation.current_key_frame(self.game_time),
                //    }.draw(item.pos.x, item.pos.y);
                //    sprite.set_scale(cgmath::vec2(3.0, 3.0));
                //    sprite.set_color(item.color.into());
                //    sprite.set_flip_x(item.flip);
                //    sprite.set_rotation(item.rotation);
                //    self.sprite.draw(&sprite, draw_params, &mut target);
                //}

                //// Draw a huge corgi!
                //let mut sprite = self.wizard_dog_run_animation.current_key_frame(self.game_time)
                //    .draw(config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0 - 50.0);
                //sprite.set_scale(cgmath::vec2(16.0, 16.0));
                //self.sprite.draw(&sprite, draw_params, &mut target);

                //// Draw win text!
                //let text = "You are the most magical corgi in all the land!\nPress R to start anew!";
                //self.text.draw_text(text, &self.font, [0.0, 0.0, 0.0],
                //                    40, 22.0, 502.0, 800, &projection, &mut target);
                //self.text.draw_text(text, &self.font, [1.0, 1.0, 1.0],
                //                    40, 20.0, 500.0, 800, &projection, &mut target);
            },
        }
    }

    fn draw_world(&mut self, dt: f32, world: &GameWorld, camera: &Camera, window: &mut Window) {
        window.set_view(View::new(Rectangle::new_sized((400, 300))));

        window.clear(Color::WHITE);

        //let draw_params = SpriteDrawParams::new()
        //    .magnify_filter(MagnifySamplerFilter::Nearest)
        //    .alpha(true);

        // Background
        // TODO: No wrap function support in quicksilver (yet)...
        //self.background.execute(|image| {
        //    let trans = Transform::scale((2.0, 2.0));
        //    window.draw_ex(&image.area().with_center((config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0)),
        //                   Background::Img(image), trans, 0.0);
        //    Ok(())
        //});
        //let pos = self.background.size();
        //let mut sprite = self.background.draw(pos.x as f32 / 2.0, pos.y as f32 / 2.0);
        //sprite.set_scale(cgmath::vec2(2.0, 2.0));
        //self.sprite.draw(&sprite,
        //                SpriteDrawParams::new()
        //                    .magnify_filter(MagnifySamplerFilter::Nearest)
        //                    .alpha(true)
        //                    .wrap_function(SamplerWrapFunction::Repeat),
        //                target);

        // Draw cat box.
        self.cat_box.execute(|image| {
            let pos = world.cat_box().pos;
            window.draw(&image.area().with_center((pos.x, pos.y)),
                        Background::Img(image));
            Ok(())
        });

        //// Draw cats!
        //for cat in &world.cats {
        //    let mut sprite = if cat.state == CatState::InPen {
        //        match cat.cat_type {
        //        self.basic_cat_idle.execute(|anim| {
        //            let image = anim.current_frame();
        //            let trans = Transform::scale((3.0, 3.0));
        //            window.draw_ex(&image.area().with_center((380.0, 340.0)),
        //                           Background::Blended(image, CAT_COLORS[0]), trans, 0.0);
        //            Ok(())
        //        });
        //            CatType::Basic => self.basic_cat_idle_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //            CatType::Kitten => self.kitten_idle_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //            CatType::Fat => self.fat_cat_idle_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //        }
        //    } else if cat.state == CatState::Cannonballing {
        //        match cat.cat_type {
        //            CatType::Basic => self.basic_cat_ball_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //            CatType::Fat => self.fat_cat_ball_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //            CatType::Kitten => {
        //                //kitten never goes into the cannonballing state
        //                self.kitten_idle_animation.current_key_frame(self.game_time)
        //                    .draw(cat.pos.x, cat.pos.y)
        //            }
        //        }
        //    } else {
        //        match cat.cat_type {
        //            CatType::Basic => self.basic_cat_walk_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //            CatType::Kitten => self.kitten_walk_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //            CatType::Fat => self.fat_cat_walk_animation.current_key_frame(self.game_time)
        //                .draw(cat.pos.x, cat.pos.y),
        //        }
        //    };
        //    sprite.set_flip_x(cat.facing == Facing::Right);
        //    let color = cgmath::vec3(cat.color[0], cat.color[1], cat.color[2])
        //        .mul_element_wise(cgmath::vec3(1.0, 1.0 - cat.normalized_jitter(), 1.0 - cat.normalized_jitter()));
        //    sprite.set_color(color);
        //    self.sprite.draw(&sprite, draw_params, target);
        //}

        // Draw dog, woof.
        match world.dog.dog_state {
            DogState::Chasing | DogState::Blinking(true) => {
                let trans = Transform::scale((if world.dog.facing == Facing::Right {
                    -1.0
                } else {
                    1.0
                }, 1.0));
                if world.dog.vel.is_zero() {
                    self.wizard_dog_idle.execute(|anim| {
                        let image = anim.current_frame();
                        window.draw_ex(&image.area().with_center((world.dog.pos.x, world.dog.pos.y)),
                                       Background::Img(image), trans, 0.0);
                        Ok(())
                    });
                } else {
                    self.wizard_dog_run.execute(|anim| {
                        let image = anim.current_frame();
                        window.draw_ex(&image.area().with_center((world.dog.pos.x, world.dog.pos.y)),
                                       Background::Img(image), trans, 0.0);
                        Ok(())
                    });
                }
            }
            DogState::Blinking(false) => {}
        }

        window.flush();
    }


    fn draw_ui(&mut self, _dt: f32, world: &GameWorld, window: &mut Window) {
        window.set_view(View::new(Rectangle::new_sized((800, 600))));

        //let projection = cgmath::ortho(0.0, config::SCREEN_SIZE.x as f32,
        //                               config::SCREEN_SIZE.y as f32, 0.0,
        //                               -1.0, 1.0);
        //let draw_params = SpriteDrawParams::new()
        //    .magnify_filter(MagnifySamplerFilter::Nearest)
        //    .alpha(true);

        //// Draw cat face next to score!
        //self.sprite.set_projection_matrix(projection);
        //let mut sprite = self.cat_face.draw(660.0, 25.0);
        //sprite.set_scale(cgmath::vec2(3.0, 3.0));
        //self.sprite.draw(&sprite, draw_params, target);
        //// Draw score text!
        //let score_text = format!("{:02}/{:02}", world.cats_scored, world.level.num_cats);
        //self.text.draw_text(&score_text, &self.font, [0.0, 0.0, 0.0],
        //                    40, 697.0, 7.0, 800, &projection, target);
        //self.text.draw_text(&score_text, &self.font, [1.0, 1.0, 1.0],
        //                    40, 695.0, 5.0, 800, &projection, target);
        //match world.game_state {
        //    GameState::Running => {
        //    },
        //    GameState::Won => {
        //        // Draw won text!
        //        let text = "Cats corralled!\nPress N to start the next level";
        //        self.text.draw_text(text, &self.font, [0.0, 0.0, 0.0],
        //                            40, 252.0, 502.0, 800, &projection, target);
        //        self.text.draw_text(text, &self.font, [1.0, 1.0, 1.0],
        //                            40, 250.0, 500.0, 800, &projection, target);
        //    },
        //    _ => {},
        //}

        window.flush();
    }
}
