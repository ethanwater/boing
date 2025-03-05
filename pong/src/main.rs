mod ground;
mod ball;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WINDOW_X: f32 = 700.;
const WINDOW_Y: f32 = 700.;
const WINDOW_TITLE: &str = "bloodsport";

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    InGameSinglePlayer,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(WINDOW_X, WINDOW_Y),
                title: WINDOW_TITLE.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(ground::GroundPlugin)
        .add_plugins(ball::BallPlugin)
        .add_systems(Startup, camera_setup)
        .insert_state(AppState::InGameSinglePlayer)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

