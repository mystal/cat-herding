use cgmath::{self, Vector2};
use quicksilver::graphics::Color;
use rand::{self, Rng};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::seq::SliceRandom;

use crate::config;
use crate::entities::CAT_COLORS;

const NUM_ITEMS: u32 = 60;

const PARTY_ITEM_SPEED: f32 = 60.0;

pub enum PartyItemKind {
    BasicCat,
    FatCat,
    Kitten,
}

impl Distribution<PartyItemKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartyItemKind {
        match rng.gen_range(0, 3) {
            0 => PartyItemKind::BasicCat,
            1 => PartyItemKind::FatCat,
            _ => PartyItemKind::Kitten,
        }
    }
}

pub struct PartyItem {
    pub kind: PartyItemKind,
    pub color: Color,
    pub rotation: f32,
    pub pos: Vector2<f32>,
    pub flip: bool,
}

impl PartyItem {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rotation_range = Uniform::new(0.0, 359.0);
        let x_range = Uniform::new(0.0, 800.0);
        let y_range = Uniform::new(-700.0, 0.0);

        PartyItem {
            kind: rng.gen::<PartyItemKind>(),
            color: *CAT_COLORS.choose(&mut rng).unwrap(),
            rotation: rotation_range.sample(&mut rng),
            pos: cgmath::vec2(x_range.sample(&mut rng),
                              y_range.sample(&mut rng)),
            flip: rng.gen(),
            //rot_dir: 
        }
    }

    fn update(&mut self, dt: f32) {
        // TODO: Update rotation
        self.rotation += match self.kind {
            PartyItemKind::BasicCat => 180.0 * dt,
            PartyItemKind::FatCat => 90.0 * dt,
            PartyItemKind::Kitten => 360.0 * dt,
        };

        // Update position.
        self.pos.y += PARTY_ITEM_SPEED * dt;

        // TODO: Wrap around once hit the bottom of the screen.
        if self.pos.y > config::SCREEN_SIZE.y as f32 + 50.0 {
            let mut rng = rand::thread_rng();
            let rotation_range = Uniform::new(0.0, 359.0);
            let x_range = Uniform::new(0.0, 800.0);
            self.pos.x = x_range.sample(&mut rng);
            self.pos.y = -50.0;
            self.rotation = rotation_range.sample(&mut rng);
        }
    }
}

pub struct Party {
    pub party_items: Vec<PartyItem>,
}

impl Party {
    pub fn new() -> Self {
        let mut party_items = Vec::with_capacity(NUM_ITEMS as usize);
        for _ in 0..NUM_ITEMS {
            party_items.push(PartyItem::new());
        }

        Party {
            party_items,
        }
    }

    pub fn update(&mut self, dt: f32) {
        for item in &mut self.party_items {
            item.update(dt);
        }
    }
}
