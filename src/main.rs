use bevy::{
    prelude::*,
    window::{PresentMode},
};
use bevy_rapier2d::prelude::*;

mod mole;
use mole::*;

mod walls;
use walls::*;

pub const PIXELS_PER_METER: f32 = 492.3;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);
const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const FLOOR_THICKNESS: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        //.init_state::<AppState>()
        //.add_systems(OnEnter(AppState::Setup),load_textures)
        //.add_systems(OnEnter(AppState::Finished), setup)
        //.insert_resource(Msaa::default())
        //.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(WallsPlugin)
        .add_plugins(MolePlugin)
        //.add_plugins(ShapePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vector2::zeros();
    rapier_config.gravity = Vec2::new(0.0, -520.0);

    commands.spawn(Camera2dBundle::default());
    /*commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + (FLOOR_THICKNESS / 2.0), 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        });*/
}

fn load_textures() {}