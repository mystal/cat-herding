use bevy::prelude::*;

use crate::{
    AppState, WORLD_SIZE,
    assets::GameAssets,
    cats::{self, Cat, CatState},
    dog::DogPlugin,
    level::NextLevelEvent,
    physics::{groups, ColliderBundle},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DogPlugin)
            .init_resource::<CatStats>()
            .add_systems(OnEnter(AppState::Playing), setup_game)
            .add_systems(Update, (
                (
                    update_cat_stats,
                    check_start_next_level,
                ).after(cats::update_cats).chain(),
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct CatStats {
    total: u32,
    in_pen: u32,
}

impl CatStats {
    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn in_pen(&self) -> u32 {
        self.in_pen
    }

    pub fn all_penned(&self) -> bool {
        self.total > 0 && self.in_pen == self.total
    }
}

#[derive(Component)]
pub struct CatBox;

fn setup_game(
    mut commands: Commands,
    mut next_level: EventWriter<NextLevelEvent>,
    assets: Res<GameAssets>,
) {
    debug!("Setup game");

    // Spawn floor.
    let floor_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(WORLD_SIZE.as_vec2() / 2.0),
            ..default()
        },
        texture: assets.floor.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        },
        ..default()
    };
    commands.spawn((
        Name::new("Floor"),
        floor_bundle,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.0,
        }
    ));

    // Spawn cat_box.
    commands.spawn((
        CatBox,
        Name::new("CatBox"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.5)),
            texture: assets.cat_box.clone(),
            ..default()
        },
        ColliderBundle::rect((60.0, 60.0).into(), groups::CATBOX, groups::CAT),
    ));

    next_level.send_default();
}

pub fn update_cat_stats(
    mut cat_stats: ResMut<CatStats>,
    cats_q: Query<&Cat>,
) {
    // TODO: Update on CatState changes instead of every frame?
    cat_stats.total = cats_q.iter().count() as u32;
    cat_stats.in_pen = cats_q.iter()
        .filter(|cat| cat.state == CatState::InPen)
        .count()
        as u32;
}

pub fn check_start_next_level(
    mut next_level: EventWriter<NextLevelEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    cat_stats: Res<CatStats>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }

    if cat_stats.all_penned() {
        next_level.send_default();
    }
}
