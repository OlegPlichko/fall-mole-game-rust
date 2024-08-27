use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use super::BottomWall;

pub struct MolePlugin;

impl Plugin for MolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_mole)
            .add_systems(Update, handle_mole_intersections_with_bottom_wall)
            .add_systems(Update, mole_movement);
    }
}

#[derive(Component)]
struct Mole;

fn spawn_mole(mut commands: Commands) {
    let mole_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.1,
        crate::PIXELS_PER_METER * -0.2,
    );

    let shape_mole = shapes::Circle {
        radius: crate::PIXELS_PER_METER * 0.03,
        center: Vec2::ZERO,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_mole),
                ..default()
            },
            Fill::color(Color::BLACK),
            Stroke::new(Color::TEAL, 2.0),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::ball(shape_mole.radius))
        .insert(Transform::from_xyz(mole_pos.x, mole_pos.y, 0.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(Mole);
}

fn handle_mole_intersections_with_bottom_wall(
    rapier_context: Res<RapierContext>,
    query_mole: Query<Entity, With<Mole>>,
    query_bottom_wall: Query<Entity, With<BottomWall>>,
    mut commands: Commands,
) {
    let mut should_spawn_mole = false;

    for entity_bottom_wall in query_bottom_wall.iter() {
        for entity_mole in query_mole.iter() {
            /* Find the intersection pair, if it exists, between two colliders. */
            if rapier_context.intersection_pair(entity_bottom_wall, entity_mole) == Some(true) {
                //commands.entity(entity_mole).despawn();
                //should_spawn_mole = true;
            }
        }
    }

    if should_spawn_mole {
        spawn_mole(commands);
    }
}

fn mole_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut moles: Query<(&mut Mole, &mut Transform), With<Mole>>,
) {
    for (mole, mut mole_transform) in moles.iter_mut() {
        let mut next_xpos = mole_transform.translation.x;
        let mut next_ypos = mole_transform.translation.y;

        if keyboard_input.pressed(KeyCode::Space) {
            println!("{:?} key pressed.", KeyCode::Space);
            next_ypos = next_ypos + crate::PIXELS_PER_METER * 0.01;
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            next_xpos = next_xpos - crate::PIXELS_PER_METER * 0.01;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            next_xpos = next_xpos + crate::PIXELS_PER_METER * 0.01;
        }

        mole_transform.translation.x = next_xpos;
        mole_transform.translation.y = next_ypos;
    }
}