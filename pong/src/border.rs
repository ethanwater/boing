use bevy::prelude::*;

use crate::components::{Border};

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, border_spawn);
    }
}

fn border_spawn(mut commands: Commands) {
    //TOP BORDER
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
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
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
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
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(10., 700.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Border);
}
