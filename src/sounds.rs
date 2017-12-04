pub use ears::{Sound, AudioController};
use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};

pub struct Sounds {
}

impl Sounds {
    pub fn new() -> Self {
        Sounds {
        }
    }

    pub fn angry_meow() -> Sound {
        let mut rng = rand::thread_rng();
        let range = Range::new(1, 4);
        let i = range.ind_sample(&mut rng); 
        match i {
            1 => Sound::new("assets/sounds/angry_cat_meow_1.wav").expect("Error on loading angry_meow_1."),
            2 => Sound::new("assets/sounds/angry_cat_meow_2.wav").expect("Error on loading angry_meow_2."),
            3 => Sound::new("assets/sounds/angry_cat_meow_3.wav").expect("Error on loading angry_meow_3."),
            4 => Sound::new("assets/sounds/angry_cat_meow_4.wav").expect("Error on loading angry_meow_4."),
            _ => Sound::new("assets/sounds/angry_cat_meow_1.wav").expect("Error on loading angry_meow_1."),
        }
    }

    pub fn basic_meow() -> Sound {
        Sound::new("assets/sounds/basic_cat_meow_1.wav").expect("Error on loading basic_meow_1.")
    }

    pub fn kitten_meow() -> Sound {
        Sound::new("assets/sounds/kitten_meow_1.wav").expect("Error on loading kitten_meow_1.")
    }

    pub fn fat_meow() -> Sound {
        Sound::new("assets/sounds/fat_cat_meow_1.wav").expect("Error on loading fat_meow_1.")
    }

    pub fn dog_yip() -> Sound {
        Sound::new("assets/sounds/dog_yip_1.wav").expect("Error on loading dog_yip_1.")
    }

    pub fn dog_woof() -> Sound {
        Sound::new("assets/sounds/dog_woof_1.wav").expect("Error on loading dog_woof_1.")
    }
}