use bevy::prelude::*;
use crate::{WindowSize, PLAYER_SIZE, components::{Player, Velocity, SpriteSize, Movement}, WHITE, PLAYER_SPEED};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
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
            translation: Vec3::new(-650., 0., 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpriteSize::from(PLAYER_SIZE))
    .insert(Movement {auto_despawn: false})
    .insert(Player)
    .insert(Velocity {y: 0.});
}

fn player_control(keyboard: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Transform), With<Player>>){
    if let Ok((mut velocity, transform)) = query.get_single_mut() {
        let translation = &transform.translation;
        velocity.y = 
        if keyboard.pressed(KeyCode::W) {
            if translation.y+85. < 350. {
                PLAYER_SPEED
            }
            else{
                0.
            }
        } 
        else if keyboard.pressed(KeyCode::S) {
            if translation.y-85. > -350. {
                -PLAYER_SPEED
            }
            else{
                0.
            }
        }
        else{
            0.
        }
    }
}

fn player_movement(mut query: Query<(&Velocity, &mut Transform), With<Player>>){
    for (velocity, mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
        translation.y += velocity.y;
    }
}
