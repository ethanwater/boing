use crate::{
    components::{BallMovement, PlayerCPU, ReactionBarrier, SpriteSize, VelocityAI},
    PLAYER_SIZE,
};
use bevy::prelude::*;

pub struct CPUPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StartupSet {
    PostStartup,
}


impl Plugin for CPUPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cpu.in_set(StartupSet::PostStartup));
    }
}

fn spawn_cpu(mut commands: Commands) {
    let _ = 
        commands
            .spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(650., 0., 10.)),
            ))
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(BallMovement {
                auto_despawn: false,
            })
            .insert(PlayerCPU)
            .insert(VelocityAI { y: 6. })
            .insert(ReactionBarrier { x: 0. });
}
