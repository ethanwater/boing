use crate::{
    components::{BallMovement, PlayerCPU, ReactionBarrier, SpriteSize, VelocityAI},
    PLAYER_SIZE, PLAYER_SPEED,
};
use bevy::prelude::*;

pub struct CPU;

impl Plugin for CPU {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn);
    }
}

fn player_spawn(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(650., 0., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(BallMovement {
            auto_despawn: false,
        })
        .insert(PlayerCPU)
        .insert(VelocityAI { y: 6. })
        .insert(ReactionBarrier { x: 0. });
}
