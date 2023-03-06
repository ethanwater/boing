use crate::{
    components::{Ball, BallMovement, BallVelocity, SpeedUp, SpriteSize},
    BALL_SIZE, INITAL_SPEED,
};
use bevy::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ball_spawn);
    }
}

fn ball_spawn(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_SIZE.0, BALL_SIZE.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SpriteSize::from(BALL_SIZE))
        .insert(BallMovement { auto_despawn: true })
        .insert(SpeedUp {
            speed: INITAL_SPEED,
        })
        .insert(Ball)
        .insert(BallVelocity { x: -5., y: 0. });
}
