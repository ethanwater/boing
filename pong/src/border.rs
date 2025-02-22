use bevy::prelude::*;
use crate::components::Border;

pub struct BorderPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StartupSet {
    PreStartup,
    Startup,
    PostStartup,
}


impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, border_spawn.in_set(StartupSet::PostStartup));
    }
}

fn border_spawn(mut commands: Commands) {
    //TOP BORDER
    let _ = 
        commands
            .spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(1400., 20.)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(0., 350., 10.)),
            ))
            .insert(Border);
    //LOW BORDER
    let _ = 
        commands
            .spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(1400., 20.)),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(0., -350., 10.)),
            ))
            .insert(Border);
}
