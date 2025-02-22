use crate::{
    components::{Ball, BallMovement, BallVelocity, SpeedUp, SpriteSize},
    BALL_SIZE, INITAL_SPEED,
};
use bevy::prelude::*;

pub struct BallPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StartupSet {
    PreStartup,
    Startup,
    PostStartup,
}


impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ball_spawn.in_set(StartupSet::PostStartup));
    }
}

fn ball_spawn(mut commands: Commands) {
    let _ = 
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_SIZE.0, BALL_SIZE.1)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(0.,0.,10.)),
        ))
        .insert(SpriteSize::from(BALL_SIZE))
        .insert(BallMovement { auto_despawn: true })
        .insert(SpeedUp {
            speed: INITAL_SPEED,
        })
        .insert(Ball)
        .insert(BallVelocity { x: -5., y: 0. });
}
