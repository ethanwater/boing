use bevy::prelude::*;
use crate::{WindowSize, PLAYER_SIZE, components::{SpriteSize, Player, Velocity2, Movement}, WHITE};

pub struct PlayerPlugin2;

impl Plugin for PlayerPlugin2{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn)
        .add_system(player_movement)
        .add_system(player_control);
    }
}

fn player_spawn(mut commands: Commands){
    commands.spawn(SpriteBundle {
        sprite: Sprite { 
            color: Color::rgb(WHITE.0,WHITE.1,WHITE.2),
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
    .insert(Movement {auto_despawn: false})
    .insert(Player)
    .insert(Velocity2 {y: 0.});
}

fn player_control(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Velocity2, With<Player>>){
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y = if keyboard.pressed(KeyCode::Up) {
            5.
        } else if keyboard.pressed(KeyCode::Down){
            -5.
        } else {
            0.
        }
    }
}

fn player_movement(mut query: Query<(&Velocity2, &mut Transform), With<Player>>){
    for (velocity, mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
        translation.y += velocity.y;
    }
}
