use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::render::mesh::VertexAttributeValues;

use crate::{
    GAME_SIZE, AppState,
    assets::GameAssets,
    dog::{DogBundle, DogPlugin},
    physics::{ColliderBundle, groups},
};

#[derive(Component)]
pub struct CatBox;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DogPlugin)
            .add_systems(OnEnter(AppState::Playing), setup_game);
    }
}

fn setup_game(
    mut commands: Commands,
    assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    debug!("Setup game");

    // Spawn camera.
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: GAME_SIZE.x as f32,
        height: GAME_SIZE.y as f32,
    };
    commands.spawn(camera_bundle);

    // Spawn dog.
    commands.spawn(DogBundle::new(Vec2::ZERO, assets.wizard_dog.clone()));

    // Spawn floor.
    let floor_image_size = images.get(&assets.floor).unwrap().size();
    let mut floor_mesh = Mesh::from(shape::Quad::default());
    if let Some(VertexAttributeValues::Float32x2(uvs)) = floor_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        for uv in uvs {
            uv[0] *= GAME_SIZE.x as f32 / floor_image_size.x as f32 / 2.0;
            uv[1] *= GAME_SIZE.y as f32 / floor_image_size.y as f32 / 2.0;
        }
    }
    let floor_bundle = ColorMesh2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: GAME_SIZE.as_vec2().extend(1.0),
            ..default()
        },
        material: materials.add(assets.floor.clone().into()),
        mesh: meshes.add(floor_mesh.into()).into(),
        ..default()
    };
    commands.spawn(floor_bundle)
        .insert(Name::new("Floor"));

    // Spawn cat_box.
    commands.spawn((
        CatBox,
        Name::new("CatBox"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-100.0, 50.0, -0.5)),
            texture: assets.cat_box.clone(),
            ..default()
        },
        ColliderBundle::rect((60.0, 60.0).into(), groups::CATBOX, groups::CAT),
    ));
}
