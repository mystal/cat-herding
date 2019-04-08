use cgmath::{self, InnerSpace, Vector2, Zero};
use quicksilver::{
    input::{ButtonState, Key},
    lifecycle::{Window},
};

use crate::entities::*;
use crate::sounds::Sounds;
use crate::level::{Level, MAX_LEVEL};
use crate::party::Party;

const MOVE_SPEED: f32 = 150.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    StartMenu,
    Credits,
    HowToPlay,
    Running,
    Won,
    GameOver,
}

pub struct GameWorld {
    pub game_state: GameState,
    pub level: Level,
    pub dog: Dog,
    pub cats: Vec<Cat>,
    pub cats_scored: u32,

    pub the_party: Party,

    pub sounds: Sounds,
}

impl GameWorld {
    pub fn new() -> Self {
        let level = Level::new(1);
        let dog = Dog {
            pos: level.cat_box.pos,
            vel: Vector2::zero(),
            size: cgmath::Vector2::new(30.0, 30.0),
            facing: Facing::Left,
            dog_state: DogState::Chasing,
            hit_time: 0.0,
            hit_frame: 0,
        };
        let cats = level.generate_cats();

        GameWorld {
            game_state: GameState::StartMenu,
            level,
            dog,
            cats,
            cats_scored: 0,
            the_party: Party::new(),
            sounds: Sounds::new(),
        }
    }

    pub fn update(&mut self, window: &Window, dt: f32) {
        //self.sounds.try_play_music();

        //dbg!(self.game_state);

        match self.game_state {
            GameState::StartMenu => self.update_start_menu(window, dt),
            GameState::Credits => self.update_credits(window, dt),
            GameState::HowToPlay => self.update_how_to_play(window, dt),
            GameState::Running => self.update_running(window, dt),
            GameState::Won => self.update_won(window, dt),
            GameState::GameOver => self.update_game_over(window, dt),
        }
    }

    fn restart(&mut self) {
        self.dog.pos = self.level.cat_box.pos;
        let cats = self.level.generate_cats();
        self.cats = cats;
        self.game_state = GameState::Running;
    }

    fn next_level(&mut self) {
        if self.level.level_num >= MAX_LEVEL {
            self.game_state = GameState::GameOver;
            return;
        }

        self.level.next_level();
        self.restart();
    }

    fn update_start_menu(&mut self, window: &Window, _dt: f32) {
        if window.keyboard()[Key::Return] == ButtonState::Held {
        //if window.keyboard()[Key::Return] == ButtonState::Pressed {
            self.game_state = GameState::HowToPlay;
        } else if window.keyboard()[Key::Tab] == ButtonState::Pressed {
            self.game_state = GameState::Credits;
        }
    }

    fn update_credits(&mut self, window: &Window, _dt: f32) {
        if window.keyboard()[Key::Return] == ButtonState::Pressed || window.keyboard()[Key::Tab] == ButtonState::Pressed {
            self.game_state = GameState::StartMenu;
        }
    }

    fn update_how_to_play(&mut self, window: &Window, _dt: f32) {
        if window.keyboard()[Key::Return] == ButtonState::Held {
        //if window.keyboard()[Key::Return] == ButtonState::Pressed {
            self.game_state = GameState::Running;
        }
    }

    fn update_game_over(&mut self, window: &Window, dt: f32) {
        if window.keyboard()[Key::R] == ButtonState::Pressed {
            self.level = Level::new(1);
            self.restart();
            return;
        }

        self.the_party.update(dt);
    }

    fn update_won(&mut self, window: &Window, dt: f32) {
        if window.keyboard()[Key::N] == ButtonState::Pressed {
            self.next_level();
        }

        self.update_running(window, dt);
    }

    fn update_running(&mut self, window: &Window, dt: f32) {
        if window.keyboard()[Key::R] == ButtonState::Pressed {
            self.restart();
            return;
        }
        if window.keyboard()[Key::Tab] == ButtonState::Pressed {
            self.next_level();
            return;
        }
        if window.keyboard()[Key::Space] == ButtonState::Pressed {
            self.dog.woof();
            self.sounds.play_woof();
        }

        // TODO: consider moving this into a poll input method
        // TODO: Clamp dog to level bounds.
        let keys = window.keyboard();
        let mut dir = Vector2::zero();
        if keys[Key::Left].is_down() && !keys[Key::Right].is_down() {
            dir.x -= 1.0;
        }
        if keys[Key::Right].is_down() && !keys[Key::Left].is_down() {
            dir.x += 1.0;
        }
        if keys[Key::Up].is_down() && !keys[Key::Down].is_down() {
            dir.y -= 1.0;
        }
        if keys[Key::Down].is_down() && !keys[Key::Up].is_down() {
            dir.y += 1.0;
        }
        if !dir.is_zero() {
            dir = dir.normalize();
        }
        if dir.x != 0.0 {
            self.dog.facing = if dir.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        }
        self.dog.vel = dir * MOVE_SPEED;
        let delta_pos = self.dog.vel * dt;
        self.dog.try_move(&self.level.bounds, delta_pos);

        self.dog.update(dt);

        let mut cats_scored = 0;
        // Cats move or run!
        for cat in &mut self.cats {
            let prev_state = cat.state.clone();
            match cat.update_state(&self.dog, &self.level.cat_box) {
                CatState::Idle => { cat.idle(&self.level.bounds, &self.level.cat_box, dt) },
                CatState::InPen => {
                    cat.in_pen(&self.level.bounds, dt);
                    cats_scored += 1;
                },
                CatState::Flee => {
                    let dir = &cat.pos - self.dog.pos;
                    cat.flee(&self.level.bounds, &dir, dt)
                },
                CatState::Jittering => {
                    cat.jitter(dt, &self.dog)
                }
                CatState::Cannonballing => {
                    if cat.cannonball(&self.level.bounds, dt, &mut self.dog) {
                        self.dog.hit();
                        self.sounds.play_yip();
                    }
                }
            }

            if cat.state == CatState::Idle || cat.state == CatState::InPen || cat.state == CatState::Flee {
                // Basic meow
                if cat.meow_time >= cat.meow_interval {
                    self.sounds.play_basic_meow();
                    cat.meow_time = 0.0;
                }
                cat.meow_time += dt;
            } else if prev_state != cat.state {
                // Angry meow
                if cat.state == CatState::Jittering || cat.state == CatState::Cannonballing {
                    self.sounds.play_random_angry_meow();
                    cat.meow_time = 0.0;
                }
            }

            if cat.velocity.x != 0.0 {
                cat.facing = if cat.velocity.x > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            }
        }

        self.cats_scored = cats_scored;

        if self.game_state != GameState::Won {
            // Check win condition!
            if self.cats_scored == self.level.num_cats && self.level.level_num < MAX_LEVEL {
                self.game_state = GameState::Won;
            } else if self.cats_scored == self.level.num_cats && self.level.level_num >= MAX_LEVEL {
                self.game_state = GameState::GameOver;
            }
        }
    }

    pub fn cat_box(&self) -> &CatBox {
        &self.level.cat_box
    }
}
