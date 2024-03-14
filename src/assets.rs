use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_asepritesheet::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

use crate::{
    AppState,
    level::Levels,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AsepritesheetPlugin::new(&["sprite.json"]),
                RonAssetPlugin::<Levels>::new(&["level.ron"]),
            ))
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::Playing)
                    .load_collection::<GameAssets>()
                    .load_collection::<SfxAssets>()
            )
            .add_systems(OnExit(AppState::Loading), assets_loaded);
    }
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "fonts/Kenney Pixel.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "ui/start_menu_background.png")]
    pub start_menu: Handle<Image>,
    #[asset(path = "ui/how_to_play.png")]
    pub how_to_play: Handle<Image>,
    #[asset(path = "level/hardwood_floor.png")]
    pub floor: Handle<Image>,
    #[asset(path = "level/cat_box.png")]
    pub cat_box: Handle<Image>,

    // #[asset(path = "wizard_dog.sprite.json")]
    pub wizard_dog: Handle<Spritesheet>,

    // #[asset(path = "basic_cat.aseprite")]
    pub basic_cat: Handle<Spritesheet>,
    // #[asset(path = "fat_cat.aseprite")]
    pub fat_cat: Handle<Spritesheet>,
    // #[asset(path = "kitten.aseprite")]
    pub kitten: Handle<Spritesheet>,

    #[asset(path = "all_levels.level.ron")]
    pub levels: Handle<Levels>,
}

#[derive(Resource, AssetCollection)]
pub struct SfxAssets {
    #[asset(path = "sounds/dog_yip_1.wav")]
    pub dog_yip: Handle<AudioSource>,
    #[asset(path = "sounds/dog_woof_1.wav")]
    pub dog_woof: Handle<AudioSource>,
    #[asset(path = "sounds/basic_cat_meow_1.wav")]
    pub basic_cat_meow: Handle<AudioSource>,
    #[asset(path = "sounds/kitten_meow_1.wav")]
    pub kitten_meow: Handle<AudioSource>,
    #[asset(path = "sounds/fat_cat_meow_1.wav")]
    pub fat_cat_meow: Handle<AudioSource>,
    #[asset(path = "sounds/trolling_doggo.ogg")]
    pub bgm: Handle<AudioSource>,
}

fn assets_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut assets: ResMut<GameAssets>,
    sfx: Res<SfxAssets>,
) {
    debug!("Loaded assets!");

    // Load Aseprite spritesheets.
    // TODO: See if we can do this during our standard load phase.
    assets.wizard_dog = load_spritesheet(&mut commands, &asset_server, "wizard_dog.sprite.json", Anchor::Center);
    assets.basic_cat = load_spritesheet(&mut commands, &asset_server, "basic_cat.sprite.json", Anchor::Center);
    assets.kitten = load_spritesheet(&mut commands, &asset_server, "kitten.sprite.json", Anchor::Center);
    assets.fat_cat = load_spritesheet(&mut commands, &asset_server, "fat_cat.sprite.json", Anchor::Center);

    audio.play(sfx.bgm.clone())
        .loop_from(24.0)
        .with_volume(0.2);
}
