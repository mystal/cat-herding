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
            Animation::from_spritesheet(image, regions.into_iter().cloned(), 6)
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

    pub fn tick_animations(&mut self, _dt: f32) -> Result<()> {
        // NOTE: If you want to actually use dt.
        //self.game_time += dt;
        //while self.game_time >= (1.0 / 60.0) {
        //    self.game_time -= 1.0 / 60.0;
        //}

        // Tick all the animations!
        self.basic_cat_walk.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.basic_cat_idle.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.basic_cat_ball.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.fat_cat_idle.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.fat_cat_walk.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.fat_cat_ball.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.kitten_idle.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.kitten_walk.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.wizard_dog_idle.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.wizard_dog_run.execute(|anim| {
            anim.tick();
            Ok(())
        })?;

        self.linda_cat.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.morgan_kitten.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.justin_spin.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.gabe_dog.execute(|anim| {
            anim.tick();
            Ok(())
        })?;
        self.guest_fox.execute(|anim| {
            anim.tick();
            Ok(())
        })?;

        Ok(())
    }

    pub fn render(&mut self, window: &mut Window, world: &GameWorld, camera: &Camera) -> Result<()> {
        match world.game_state {
            GameState::StartMenu => {
                // Draw start menu splash screen!
                self.start_menu.execute(|image| {
                    window.draw(&image.area().with_center((config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0)),
                                Background::Img(image));
                    Ok(())
                })?;

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
                window.clear(Color::WHITE)?;

                // Draw our sprites!
                self.linda_cat.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((-4.0, 4.0));
                    window.draw_ex(&image.area().with_center((200.0, 50.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                })?;

                self.morgan_kitten.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((-4.0, 4.0));
                    window.draw_ex(&image.area().with_center((200.0, 160.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                })?;

                self.justin_spin.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((-4.0, 4.0));
                    window.draw_ex(&image.area().with_center((200.0, 270.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                })?;

                self.gabe_dog.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((-3.5, 3.5));
                    window.draw_ex(&image.area().with_center((200.0, 380.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                })?;

                self.guest_fox.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((-3.0, 3.0));
                    window.draw_ex(&image.area().with_center((200.0, 490.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                })?;

                // Draw our names!
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

                // Draw blinking text!
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
                })?;

                // Draw corgi idle animation
                self.wizard_dog_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((4.0, 4.0));
                    window.draw_ex(&image.area().with_center((670.0, 50.0)),
                                   Background::Img(image), trans, 0.0);
                    Ok(())
                })?;

                // Draw cat animations
                self.basic_cat_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((3.0, 3.0));
                    window.draw_ex(&image.area().with_center((380.0, 340.0)),
                                   Background::Blended(image, CAT_COLORS[0]), trans, 0.0);
                    Ok(())
                })?;

                self.fat_cat_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((3.0, 3.0));
                    window.draw_ex(&image.area().with_center((480.0, 340.0)),
                                   Background::Blended(image, CAT_COLORS[3]), trans, 0.0);
                    Ok(())
                })?;

                self.kitten_idle.execute(|anim| {
                    let image = anim.current_frame();
                    let trans = Transform::scale((3.0, 3.0));
                    window.draw_ex(&image.area().with_center((580.0, 340.0)),
                                   Background::Blended(image, CAT_COLORS[2]), trans, 0.0);
                    Ok(())
                })?;

                // Draw blinking text!
                //if self.game_time.fract() < 0.5 {
                //    self.text.draw_text("Press Enter to play!", &self.font, [0.0, 0.0, 0.0],
                //                        40, 452.0, 542.0, 500, &projection, &mut target);
                //    self.text.draw_text("Press Enter to play!", &self.font, [1.0, 1.0, 1.0],
                //                        40, 450.0, 540.0, 500, &projection, &mut target);
                //}

                window.flush()?;
            },
            GameState::Running | GameState::Won => {
                self.draw_world(world, camera, window)?;
                self.draw_ui(world, window)?;
            },
            GameState::GameOver => {
                self.draw_world(world, camera, window)?;

                window.set_view(View::new(Rectangle::new_sized((800, 600))));

                // Draw the party!
                for item in &world.the_party.party_items {
                    let anim = match item.kind {
                        PartyItemKind::BasicCat => &mut self.basic_cat_idle,
                        PartyItemKind::FatCat => &mut self.fat_cat_idle,
                        PartyItemKind::Kitten => &mut self.kitten_idle,
                    };
                    anim.execute(|anim| {
                        let image = anim.current_frame();
                        let trans = Transform::rotate(item.rotation) * Transform::scale((if item.flip {
                            -3.0
                        } else {
                            3.0
                        }, 3.0));
                        window.draw_ex(&image.area().with_center((item.pos.x, item.pos.y)),
                                       Background::Blended(image, item.color), trans, 0);
                        Ok(())
                    })?;
                }

                // Draw a huge corgi!
                self.wizard_dog_run.execute(|anim| {
                    let image = anim.current_frame();
                    let pos = (config::SCREEN_SIZE.x as f32 / 2.0, config::SCREEN_SIZE.y as f32 / 2.0 - 50.0);
                    let trans = Transform::scale((16.0, 16.0));
                    window.draw_ex(&image.area().with_center(pos),
                                   Background::Img(image), trans, 0);
                    Ok(())
                })?;

                // Draw win text!
                //let text = "You are the most magical corgi in all the land!\nPress R to start anew!";
                //self.text.draw_text(text, &self.font, [0.0, 0.0, 0.0],
                //                    40, 22.0, 502.0, 800, &projection, &mut target);
                //self.text.draw_text(text, &self.font, [1.0, 1.0, 1.0],
                //                    40, 20.0, 500.0, 800, &projection, &mut target);

                window.flush()?;
            },
        }

        Ok(())
    }

    fn draw_world(&mut self, world: &GameWorld, camera: &Camera, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        window.set_view(View::new(Rectangle::new_sized((400, 300))));

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
            window.draw_ex(&image.area().with_center((pos.x, pos.y)),
                           Background::Img(image), Transform::IDENTITY, 1);
            Ok(())
        })?;

        // Draw cats!
        for cat in &world.cats {
            let anim = match (cat.state, cat.cat_type) {
                (CatState::InPen, CatType::Basic) => &mut self.basic_cat_idle,
                (CatState::InPen, CatType::Kitten) => &mut self.kitten_idle,
                (CatState::InPen, CatType::Fat) => &mut self.fat_cat_idle,

                (CatState::Cannonballing, CatType::Basic) => &mut self.basic_cat_ball,
                // NOTE: Kitten never goes into the cannonballing state.
                (CatState::Cannonballing, CatType::Kitten) => &mut self.kitten_idle,
                (CatState::Cannonballing, CatType::Fat) => &mut self.fat_cat_ball,

                (_, CatType::Basic) => &mut self.basic_cat_walk,
                (_, CatType::Kitten) => &mut self.kitten_walk,
                (_, CatType::Fat) => &mut self.fat_cat_walk,
            };
            let trans = Transform::scale((if cat.facing == Facing::Right {
                -1.0
            } else {
                1.0
            }, 1.0));
            let color = cat.color
                .multiply(Color { r: 1.0, g: 1.0 - cat.normalized_jitter(), b: 1.0 - cat.normalized_jitter(), a: 1.0 });
            anim.execute(|anim| {
                let image = anim.current_frame();
                window.draw_ex(&image.area().with_center((cat.pos.x, cat.pos.y)),
                               Background::Blended(image, color), trans, 2);
                Ok(())
            })?;
        }

        // Draw dog, woof.
        match world.dog.dog_state {
            DogState::Chasing | DogState::Blinking(true) => {
                let anim = if world.dog.vel.is_zero() {
                    &mut self.wizard_dog_idle
                } else {
                    &mut self.wizard_dog_run
                };
                let trans = Transform::scale((if world.dog.facing == Facing::Right {
                    -1.0
                } else {
                    1.0
                }, 1.0));
                anim.execute(|anim| {
                    let image = anim.current_frame();
                    window.draw_ex(&image.area().with_center((world.dog.pos.x, world.dog.pos.y)),
                                   Background::Img(image), trans, 3);
                    Ok(())
                })?;
            }
            DogState::Blinking(false) => {}
        }

        window.flush()?;

        Ok(())
    }


    fn draw_ui(&mut self, world: &GameWorld, window: &mut Window) -> Result<()> {
        window.set_view(View::new(Rectangle::new_sized((800, 600))));

        // Draw cat face next to score!
        let trans = Transform::scale((3.0, 3.0));
        self.cat_face.execute(|image| {
            window.draw_ex(&image.area().with_center((660.0, 25.0)),
                           Background::Img(image), trans, 0);
            Ok(())
        })?;

        // Draw score text!
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

        window.flush()?;

        Ok(())
    }
}
