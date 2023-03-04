use bevy::prelude::*;

use crate::{components::{Border, SpriteSize}, BLACK, WHITE};

pub struct BorderPlugin;

impl Plugin for BorderPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, border_spawn)
        .add_system(goose);
    }
}

fn border_spawn(mut commands: Commands){
    //TOP BORDER
    commands.spawn(SpriteBundle {
        sprite: Sprite { 
            color: Color::rgb(WHITE.0,WHITE.1,WHITE.2),
            custom_size: Some(Vec2::new(1400., 20.)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 350., 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    //LOW BORDER
    .insert(Border);
    commands.spawn(SpriteBundle {
        sprite: Sprite { 
            color: Color::rgb(WHITE.0,WHITE.1,WHITE.2),
            custom_size: Some(Vec2::new(1400., 20.)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., -350., 10.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Border);
}

fn goose(mut query: Query<(&mut Transform), With<Border>>){
    for (mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
    }
}