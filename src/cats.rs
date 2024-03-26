use std::{f32::consts::PI, ops::Range, time::Duration};

use bevy::prelude::*;
use bevy_asepritesheet::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::RapierContext;

use crate::{
    WORLD_SIZE, AppState,
    assets::SfxAssets,
    dog::Dog,
    game::CatBox,
    physics::{self, groups, ColliderBundle, MovementBounds, Velocity},
};

// TODO: Kinda sucks to hard-code these, but I'm too lazy to figure out how to pipe in them right
// now.
pub const IDLE_ANIM: AnimHandle = AnimHandle::from_index(1);
pub const WALK_ANIM: AnimHandle = AnimHandle::from_index(0);
pub const ATTACK_ANIM: AnimHandle = AnimHandle::from_index(2);

pub const CAT_BOUNDS: f32 = 15.0;

const FLEE_RANGE: f32 = 70.0;
const FLEE_BUFFER: f32 = 20.0;

const JITTER_TIME: f32 = 1.0;
const CANNONBALL_TIME: f32 = 1.25;
const CANNONBALL_SPEED: f32 = 240.0;

const MEOW_RANGE: Range<f32> = 4.0..8.0;

pub struct CatsPlugin;

impl Plugin for CatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_cats.before(physics::update_movement),
                (cat_animation, cat_color).after(update_cats),
                cat_meows,
                init_cat_color,
            ).run_if(in_state(AppState::Playing)));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CatKind {
    Basic,
    Kitten,
    Chonk,
}

impl CatKind {
    fn walk_speed(&self) -> f32 {
        match self {
            CatKind::Basic => 50.0,
            CatKind::Kitten => 60.0,
            CatKind::Chonk => 35.0,
        }
    }

    fn walk_turn_radius(&self) -> f32 {
        match self {
            CatKind::Basic => 9.0,
            CatKind::Kitten => 12.0,
            CatKind::Chonk => 6.0,
        }
    }

    fn flee_speed(&self) -> f32 {
        match self {
            CatKind::Basic => 175.0,
            CatKind::Kitten => 250.0,
            CatKind::Chonk => 100.0,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CatState {
    Wander { accel_angle: f32 },
    Flee,
    Jittering { timer: Timer },
    Cannonballing { timer: Timer },
    InPen,
}

#[derive(Component)]
pub struct Cat {
    pub kind: CatKind,
    pub state: CatState,
    color: Color,
    meow_timer: Timer,
}

impl Cat {
    fn new(kind: CatKind) -> Self {
        let meow_time = MEOW_RANGE.start + (fastrand::f32() * (MEOW_RANGE.end - MEOW_RANGE.start));
        Self {
            kind,
            state: CatState::Wander { accel_angle: fastrand::f32() * 2.0 * PI },
            color: Color::WHITE,
            meow_timer: Timer::from_seconds(meow_time, TimerMode::Once),
        }
    }

    fn reset_meow(&mut self) {
        let meow_time = MEOW_RANGE.start + (fastrand::f32() * (MEOW_RANGE.end - MEOW_RANGE.start));
        self.meow_timer = Timer::from_seconds(meow_time, TimerMode::Once);
    }
}

#[derive(Component)]
pub struct Annoyance {
    current: f32,
    annoyance_rate: f32,
    calming_rate: f32,
}

impl Annoyance {
    fn from_cat_kind(kind: CatKind) -> Self {
        let time_to_annoy = match kind {
            CatKind::Basic => 1.0,
            CatKind::Kitten => -1.0,
            CatKind::Chonk => 0.67,
        };
        let time_to_calm = match kind {
            CatKind::Basic => 1.3,
            CatKind::Kitten => 0.1,
            CatKind::Chonk => 2.0,
        };
        Self::new(time_to_annoy, time_to_calm)
    }

    fn new(time_to_annoy: f32, time_to_calm: f32) -> Self {
        Self {
            current: 0.0,
            annoyance_rate: 1.0 / time_to_annoy,
            calming_rate: 1.0 / time_to_calm,
        }
    }

    fn increase(&mut self, dt: Duration) -> bool {
        self.current += self.annoyance_rate * dt.as_secs_f32();
        self.current = self.current.clamp(0.0, 1.0);
        self.is_annoyed()
    }

    fn decrease(&mut self, dt: Duration) {
        self.current -= self.calming_rate * dt.as_secs_f32();
        self.current = self.current.clamp(0.0, 1.0);
    }

    fn reset(&mut self) {
        self.current = 0.0;
    }

    fn is_annoyed(&self) -> bool {
        self.current >= 1.0
    }
}

#[derive(Bundle)]
pub struct CatBundle {
    cat: Cat,
    annoyance: Annoyance,
    name: Name,
    sprite: AnimatedSpriteBundle,
    velocity: Velocity,
    collider: ColliderBundle,
    bounds: MovementBounds,
}

impl CatBundle {
    fn new(name: &'static str, pos: Vec2, spritesheet: Handle<Spritesheet>, kind: CatKind) -> Self {
        Self {
            cat: Cat::new(kind),
            annoyance: Annoyance::from_cat_kind(kind),
            name: Name::new(name),
            sprite: AnimatedSpriteBundle {
                animator: SpriteAnimator::from_anim(IDLE_ANIM),
                spritesheet,
                sprite_bundle: SpriteSheetBundle {
                    sprite: Sprite {
                        flip_x: fastrand::bool(),
                        ..default()
                    },
                    transform: Transform::from_translation(pos.extend(2.0)),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::default(),
            collider: ColliderBundle::rect(Vec2::new(30.0, 30.0), groups::CAT, groups::DOG | groups::CATBOX),
            bounds: MovementBounds {
                min: -(WORLD_SIZE.as_vec2() / 2.0) + Vec2::splat(CAT_BOUNDS),
                max: (WORLD_SIZE.as_vec2() / 2.0) - Vec2::splat(CAT_BOUNDS),
            },
        }
    }

    pub fn basic(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self::new("BasicCat", pos, spritesheet, CatKind::Basic)
    }

    pub fn kitten(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self::new("KittenCat", pos, spritesheet, CatKind::Kitten)
    }

    pub fn chonk(pos: Vec2, spritesheet: Handle<Spritesheet>) -> Self {
        Self::new("ChonkCat", pos, spritesheet, CatKind::Chonk)
    }
}

pub fn update_cats(
    time: Res<Time>,
    audio: Res<Audio>,
    rapier_ctx: Res<RapierContext>,
    sounds: Res<SfxAssets>,
    mut cat_q: Query<(Entity, &mut Cat, &mut Annoyance, &Transform, &mut Velocity)>,
    dog_q: Query<(&Dog, &GlobalTransform), Without<Cat>>,
    cat_box_q: Query<(Entity, &GlobalTransform), With<CatBox>>,
) {
    let dt = time.delta();

    let (dog_recovering, dog_pos) = dog_q.get_single()
        .map(|(dog, trans)| (dog.is_recovering(), Some(trans.translation().truncate())))
        .unwrap_or((false, None));
    let (cat_box, catbox_pos) = cat_box_q.get_single()
        .map(|(cat_box, trans)| (Some(cat_box), Some(trans.translation().truncate())))
        .unwrap_or_default();
    for (entity, mut cat, mut annoyance, transform, mut velocity) in cat_q.iter_mut() {
        let pos = transform.translation.truncate();

        let in_pen = cat_box.map(|cat_box|
            rapier_ctx.intersection_pair(entity, cat_box) == Some(true))
            .unwrap_or(false);
        let dog_in_range = dog_pos.map(|dog_pos|
            pos.distance_squared(dog_pos) < FLEE_RANGE.powi(2))
            .unwrap_or(false);
        let dog_out_of_range = dog_pos.map(|dog_pos|
            pos.distance_squared(dog_pos) > (FLEE_RANGE + FLEE_BUFFER).powi(2))
            .unwrap_or(true);

        // Update cat state first.
        match &cat.state {
            CatState::Wander { .. } => {
                if in_pen {
                    cat.state = CatState::InPen;
                } else if !dog_recovering && dog_in_range {
                    cat.state = CatState::Flee;
                }
            }
            CatState::Flee => {
                if in_pen {
                    cat.state = CatState::InPen;
                } else if annoyance.is_annoyed() {
                    cat.state = CatState::Jittering {
                        timer: Timer::from_seconds(JITTER_TIME, TimerMode::Once),
                    };
                    **velocity = Vec2::ZERO;
                    let sound = fastrand::choice(sounds.angry_cat.iter()).unwrap();
                    audio.play(sound.clone())
                        .with_volume(0.6);
                } else if !dog_recovering && dog_out_of_range {
                    // Start wandering facing the direction we were fleeing.
                    let accel_angle = velocity.to_angle() + PI;
                    cat.state = CatState::Wander { accel_angle };
                }
            }
            CatState::Jittering { timer } => {
                if timer.finished() {
                    cat.state = CatState::Cannonballing {
                        timer: Timer::from_seconds(CANNONBALL_TIME, TimerMode::Once),
                    };
                    **velocity = if let Some(dog_pos) = dog_pos {
                        // Cannonball towards dog.
                        let move_dir = (dog_pos - pos).normalize_or_zero();
                        move_dir * CANNONBALL_SPEED
                    } else {
                        Vec2::ZERO
                    };
                }
            }
            CatState::Cannonballing { timer } => {
                if timer.finished() {
                    // Start wandering facing the direction we were fleeing.
                    let accel_angle = velocity.to_angle() + PI;
                    cat.state = CatState::Wander { accel_angle };
                    annoyance.reset();
                }
            }
            CatState::InPen => {},
        }

        // Perform cat state logic.
        match &cat.state {
            CatState::Flee => {
                annoyance.increase(dt);
            }
            CatState::Wander { .. } | CatState::InPen => {
                annoyance.decrease(dt);
            }
            _ => {}
        }
        match &mut cat.state {
            CatState::Wander { accel_angle }=> {
                // TODO: Wander logic.

                // Update the desired wander acceleration by a random amount.
                let accel_angle_delta = ((fastrand::f32() * 2.0) - 1.0) * 18.0;
                *accel_angle += accel_angle_delta.to_radians();

                // Update velocity to move a bit more towards the desired angle.
                **velocity += Vec2::from_angle(*accel_angle) * cat.kind.walk_turn_radius();
                // And set speed to walk speed.
                **velocity = velocity.normalize() * cat.kind.walk_speed();

                // apply repulsive force if we're close to the cat box
                if let Some(catbox_pos) = catbox_pos {
                    let box_to_cat = pos - catbox_pos;
                    // if box_to_cat.magnitude() < (cat_box.size.x + self.radius) {
                    //     self.velocity = (self.velocity + box_to_cat.normalize() * 150.0 / box_to_cat.magnitude()).normalize() * self.speed / 3.0;
                    // }
                    // v = self.velocity;

                    // **velocity = Vec2::ZERO;
                }
            }
            CatState::Flee => {
                if let Some(dog_pos) = dog_pos {
                    let flee_dir = (pos - dog_pos).normalize_or_zero();
                    **velocity = flee_dir * cat.kind.flee_speed();
                }
            }
            CatState::Jittering { timer } => {
                timer.tick(dt);
            }
            CatState::Cannonballing { timer } => {
                timer.tick(dt);
            }
            CatState::InPen => {
                **velocity = Vec2::ZERO;
            }
        }
    }
}

fn cat_animation(
    mut cat_q: Query<(&mut SpriteAnimator, &mut Sprite, &Cat, &Velocity)>,
) {
    // Update which animation is playing based on state and velocity.
    for (mut animator, mut sprite, cat, velocity) in cat_q.iter_mut() {
        match &cat.state {
            CatState::Wander { .. }=> {
                if **velocity == Vec2::ZERO {
                    if !animator.is_cur_anim(IDLE_ANIM) {
                        animator.set_anim(IDLE_ANIM);
                    }
                } else {
                    if !animator.is_cur_anim(WALK_ANIM) {
                        animator.set_anim(WALK_ANIM);
                    }
                    if velocity.x != 0.0 {
                        sprite.flip_x = velocity.x > 0.0;
                    }
                }
            }
            CatState::Flee => {
                if !animator.is_cur_anim(WALK_ANIM) {
                    animator.set_anim(WALK_ANIM);
                }
                if **velocity != Vec2::ZERO {
                    sprite.flip_x = velocity.x > 0.0;
                }
            }
            CatState::Jittering { .. } => {
                if !animator.is_cur_anim(IDLE_ANIM) {
                    animator.set_anim(IDLE_ANIM);
                }
            }
            CatState::Cannonballing { .. } => {
                if !animator.is_cur_anim(ATTACK_ANIM) {
                    animator.set_anim(ATTACK_ANIM);
                }
            }
            CatState::InPen => {
                if !animator.is_cur_anim(IDLE_ANIM) {
                    animator.set_anim(IDLE_ANIM);
                }
            }
        }

        // TODO: Jitter sprite!
        // match &cat.state {
        //     CatState::Jittering { .. } => {
        //         let dir = Vec2::from_angle(fastrand::f32() * TAU);
        //         let anchor = dir * ((3.0 / 16.0) * 0.5);
        //         sprite.anchor = Anchor::Custom(anchor);
        //     }
        //     _ => {
        //         sprite.anchor = Anchor::Center;
        //     }
        // }
    }
}

fn cat_color(
    mut cat_q: Query<(&Annoyance, &Cat, &mut Sprite), (With<Cat>, Changed<Annoyance>)>,
) {
    for (annoyance, cat, mut sprite) in cat_q.iter_mut() {
        let base_linear = cat.color.rgb_linear_to_vec3();
        let red_linear = Color::RED.rgb_linear_to_vec3();
        let color_linear = base_linear.lerp(red_linear, annoyance.current);
        sprite.color = Color::rgb_linear_from_array(color_linear);
    }
}

fn cat_meows(
    time: Res<Time>,
    audio: Res<Audio>,
    sounds: Res<SfxAssets>,
    mut cat_q: Query<&mut Cat>,
) {
    let dt = time.delta();
    for mut cat in cat_q.iter_mut() {
        if matches!(cat.state, CatState::Jittering { .. } | CatState::Cannonballing { .. }) {
            // Don't tick in annoyed states.
            continue;
        }

        if cat.meow_timer.tick(dt).finished() {
            audio.play(match cat.kind {
                CatKind::Basic => sounds.basic_cat_meow.clone(),
                CatKind::Kitten => sounds.kitten_meow.clone(),
                CatKind::Chonk => sounds.fat_cat_meow.clone(),
            });
            cat.reset_meow();
        }
    }
}

pub const CAT_COLORS: &[[f32; 3]] = &[
    [203.0 / 255.0, 219.0 / 255.0, 252.0 / 255.0], // The default purple blue
    [189.0 / 255.0, 245.0 / 255.0, 242.0 / 255.0], // Robin's egg blue-ish
    [174.0 / 255.0, 245.0 / 255.0, 184.0 / 255.0], // Pastel green.
    [255.0 / 255.0, 193.0 / 255.0, 229.0 / 255.0], // Not quite but sort of pink.
];

fn init_cat_color(
    mut cat_q: Query<(&mut Sprite, &mut Cat), Added<Sprite>>,
) {
    for (mut sprite, mut cat) in cat_q.iter_mut() {
        let color = fastrand::choice(CAT_COLORS).unwrap();
        sprite.color = Color::rgb_from_array(*color);
        cat.color = Color::rgb_from_array(*color);
    }
}
