use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

pub struct GroundPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StartupSet {
    PostStartup,
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground.in_set(StartupSet::PostStartup));
    }
}

fn spawn_ground(mut commands: Commands) {
    //..rightwall
    commands
        .spawn(Collider::cuboid(15.0, 700.0))
        .insert(Transform::from_xyz(-350.0, 0.0, 0.0));
    //..leftwall
    commands
        .spawn(Collider::cuboid(15.0, 700.0))
        .insert(Transform::from_xyz(350.0, 0.0, 0.0));
    //..floor
    commands
        .spawn(Collider::cuboid(700.0, 15.0))
        .insert(Transform::from_xyz(0.0, -350.0, 0.0));
}
