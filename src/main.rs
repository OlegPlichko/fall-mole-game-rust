use bevy::{
    prelude::*,
    window::{PresentMode},
};
use bevy_prototype_lyon::prelude::*;
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

fn main() {
    let mut app = App::new();
    // Use `SubApp::new()` instead of `App::new()`.
    let mut sub_app = SubApp::new();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fall Mole".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup),load_textures)
        .add_systems(OnEnter(AppState::Finished), setup)
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(WallsPlugin)
        .add_plugins(MolePlugin)
        .add_plugins(ShapePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vector2::zeros();
    rapier_config.gravity = Vec2::new(0.0, -520.0);

    commands.spawn(Camera2dBundle::default());
}

fn load_textures() {}