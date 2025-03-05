use std::time::Duration;

use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
};
use bevy_rapier2d::prelude::{Collider, Restitution, RigidBody, Velocity};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_resource_cursor).add_systems(
            Update,
            (
                spawn_ball.run_if(input_pressed(KeyCode::Space)),
                cursor_position,
            ),
        );
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cursor_position: Res<CursorPosition>,
) {
    let shape = Circle::new(10.);
    let color = Color::srgb(1., 0., 0.);
    let mesh = meshes.add(shape);
    let material = materials.add(color);

    commands
        .spawn((Mesh2d(mesh.clone()), MeshMaterial2d(material.clone())))
        .insert(RigidBody::default())
        .insert(Collider::ball(10.))
        .insert(Restitution::coefficient(0.7));
}

use bevy::window::PrimaryWindow;

#[derive(Resource)]
struct CursorPosition(Vec2);

#[derive(Component)]
struct MainCamera;

fn cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_windows.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        cursor_position.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

fn init_resource_cursor(mut commands: Commands) {
    commands.insert_resource(CursorPosition {
        0: Vec2 { x: 0., y: 0. },
    });
}
