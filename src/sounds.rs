use quicksilver::{
    lifecycle::Asset,
    sound::Sound,
};
use rand::seq::SliceRandom;

pub struct Sounds {
    pub intro_music: Asset<Sound>,
    pub background_music: Asset<Sound>,
    pub music_playing: bool,

    pub yip_sound: Asset<Sound>,
    pub woof_sound: Asset<Sound>,

    pub basic_meow: Asset<Sound>,
    pub kitten_meow: Asset<Sound>,
    pub fat_meow: Asset<Sound>,
    pub angry_meows: Vec<Asset<Sound>>,
}

impl Sounds {
    pub fn new() -> Self {
        let mut background_music = Asset::new(Sound::load("sounds/trolling_doggo_loop.wav"));
        //background_music.set_looping(true);

        let angry_meows = vec![
            Asset::new(Sound::load("sounds/angry_cat_meow_1.wav")),
            Asset::new(Sound::load("sounds/angry_cat_meow_2.wav")),
            Asset::new(Sound::load("sounds/angry_cat_meow_3.wav")),
            Asset::new(Sound::load("sounds/angry_cat_meow_4.wav")),
        ];

        Sounds {
            intro_music: Asset::new(Sound::load("sounds/trolling_doggo.wav")),
            background_music,
            music_playing: false,

            yip_sound: Asset::new(Sound::load("sounds/dog_yip_1.wav")),
            woof_sound: Asset::new(Sound::load("sounds/dog_woof_1.wav")),

            basic_meow: Asset::new(Sound::load("sounds/basic_cat_meow_1.wav")),
            kitten_meow: Asset::new(Sound::load("sounds/kitten_meow_1.wav")),
            fat_meow: Asset::new(Sound::load("sounds/fat_cat_meow_1.wav")),
            angry_meows,
        }
    }

    pub fn try_play_music(&mut self) {
        if !self.music_playing {
            let music_playing = &mut self.music_playing;
            self.intro_music.execute(|sound| {
                sound.play();
                *music_playing = true;
                Ok(())
            });
        }
    }

    pub fn play_yip(&mut self) {
        self.yip_sound.execute(|sound| sound.play());
    }

    pub fn play_woof(&mut self) {
        self.woof_sound.execute(|sound| sound.play());
    }

    pub fn play_basic_meow(&mut self) {
        self.basic_meow.execute(|sound| sound.play());
    }

    pub fn play_kitten_meow(&mut self) {
        self.kitten_meow.execute(|sound| sound.play());
    }

    pub fn play_fat_meow(&mut self) {
        self.fat_meow.execute(|sound| sound.play());
    }

    pub fn play_random_angry_meow(&mut self) {
        let mut rng = rand::thread_rng();
        self.angry_meows.choose_mut(&mut rng)
            .expect("Could not get a random angry meow sound")
            .execute(|sound| sound.play());
    }
}
