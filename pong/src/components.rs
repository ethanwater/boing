use bevy::prelude::{Component, Vec2};
//Border
#[derive(Component)]
pub struct Border;

//Player
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Velocity {
    pub y: f32,
}

#[derive(Component)]
pub struct PlayerCPU;
#[derive(Component)]
pub struct VelocityAI {
    pub y: f32,
}
#[derive(Component)]
pub struct ReactionBarrier {
    pub x: f32,
}

//Ball
#[derive(Component)]
pub struct Ball;
#[derive(Component)]
pub struct BallVelocity {
    pub x: f32,
    pub y: f32,
}
#[derive(Component)]
pub struct SpeedUp {
    pub speed: f32,
}
#[derive(Component)]
pub struct BallMovement {
    pub auto_despawn: bool,
}

//Sprite Size
#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}


