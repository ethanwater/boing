use crate::{
    components::{BallMovement, Player, SpriteSize, Velocity},
    PLAYER_SIZE,
};
use bevy::prelude::*;


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StartupSet {
    PreStartup,
    Startup,
    PostStartup,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_spawn, player_movement).in_set(StartupSet::PostStartup));
    }
}

fn player_spawn(mut commands: Commands) {
    let _ =
        commands
        .spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(-650., 0., 10.)),
        ))
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(BallMovement {
            auto_despawn: false,
        })
        .insert(Player)
        .insert(Velocity { y: 0. });
}

fn player_movement(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y += velocity.y;
    }
}
