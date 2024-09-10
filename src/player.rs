use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::{climbing::Climber, inventory::Inventory};
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle("Run (32x32).png", 32, 32, 12, 1, 0, 0, 0)]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Inventory,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Component)]
struct AnimationConfig {
    first: usize,
    last: usize,
    step: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(mut commands: Commands) {
    let animation_indices = AnimationConfig { first: 1, last: 11, step: 1 };
    commands.spawn((
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animation_indices,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query_atlas: Query<&mut TextureAtlas, With<Player>>,
    mut query_timer: Query<(&AnimationConfig, &mut AnimationTimer)>,
) {
    for mut atlas in &mut query_atlas {
        for (indices, mut timer) in &mut query_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + indices.step
                };
            }
        }
    }
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &mut Sprite, &GroundDetection), With<Player>>, 
) {
    for (mut velocity, mut climber, mut sprite, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::ArrowRight) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::ArrowLeft) { 1. } else { 0. };
        if left > 0. {
            sprite.flip_x = true;
        } else if right > 0. {
            sprite.flip_x = false;
        }

        velocity.linvel.x = (right - left) * 100.;

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::ArrowUp) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::ArrowDown) { 1. } else { 0. };

            velocity.linvel.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 300.;
            climber.climbing = false;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement)
            .add_systems(Update, animate_sprite)
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
