#![allow(unused)]
use clap::Parser;
use bevy::{
    app::AppExit,
    ecs::system::Command,
    input::keyboard,
    math::Vec3Swizzles,
    prelude::*,
    sprite::collide_aabb::{self, collide},
};
use components::{
    Ball, BallMovement, BallVelocity, Player, PlayerCPU, ReactionBarrier, SpeedUp, SpriteSize,
    Velocity, Velocity2, VelocityAI,
};
use border::BorderPlugin;
use ball::BallPlugin;
use cpu::CPU;
use player::PlayerPlugin;
use player2::PlayerPlugin2;
use std::f32::consts::PI;
mod ball;
mod border;
mod components;
mod cpu;
mod player;
mod player2;

const PLAYER_SIZE: (f32, f32) = (20., 125.);
const BALL_SIZE: (f32, f32) = (20., 20.);
const MAX_BOUNCE_ANGLE: f32 = (5. * PI) / 18.;
const PLAYER_SPEED: f32 = 12.;
const INITIAL_CPU_SPEED: f32 = 5.;
const MAX_SPEED_UP: f32 = 17.;
const INITAL_SPEED: f32 = 5.;

#[derive(Parser, Default, Debug)]
#[clap(author="angelshatepop", version, about="pong")]
struct Args {
    choice: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGameTwoPlayer,
    InGameSinglePlayer,
    Paused,
}
#[derive(Resource, Component)]
struct Score1 {
    score: usize,
}

#[derive(Resource, Component)]
struct Score2 {
    score: usize,
}
fn main(){
    let args = Args::parse();
    start_game(args.choice)
}

fn start_game(choice: u32){
    if choice == 1{
        single_player();
    }
    else if choice == 2{
        two_player();
    }
    else{
        panic!("{} is not an option, can only be:\n1 (SinglepPlayer)\n2 (TwoPlayer", choice);
    }
}

fn single_player() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1400.0,
                height: 700.0,
                title: "pong".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(game_setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(CPU)
        .add_plugin(BorderPlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(Score1 { score: 0 })
        .insert_resource(Score2 { score: 0 })
        .add_state(AppState::InGameSinglePlayer)

        .add_system_set(
            SystemSet::on_update(AppState::InGameSinglePlayer)
                .with_system(player_control)
                .with_system(cpu_control)
                .with_system(ball_collision_system)
                .with_system(ball_movement)
                .with_system(update_score1)
                .with_system(update_score2)
                .with_system(exit_app)
                .with_system(pause),
        )

        .add_system_set(
            SystemSet::on_update(AppState::Paused)
                .with_system(play)
                .with_system(exit_app),
        )
        .run();
}

fn two_player() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1400.0,
                height: 700.0,
                title: "pong".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(game_setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlayerPlugin2)
        .add_plugin(BallPlugin)
        .add_plugin(BorderPlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(Score1 { score: 0 })
        .insert_resource(Score2 { score: 0 })
        .add_state(AppState::InGameTwoPlayer)

        .add_system_set(
            SystemSet::on_update(AppState::InGameTwoPlayer)
                .with_system(player_control)
                .with_system(player_control2)
                .with_system(ball_collision_system)
                .with_system(ball_movement)
                .with_system(update_score1)
                .with_system(update_score2)
                .with_system(exit_app)
                .with_system(pause),
        )

        .add_system_set(
            SystemSet::on_update(AppState::Paused)
                .with_system(play)
                .with_system(exit_app),
        )
        .run();
}


fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/PixeloidSansBold-GOjpP.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(620.0),
                ..default()
            },
            ..default()
        }),
        Score1 { score: 0 },
    ));
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/PixeloidSansBold-GOjpP.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(616.0),
                ..default()
            },
            ..default()
        }),
        Score2 { score: 0 },
    ));
}

pub fn player_control(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    if let Ok((mut player_velocity, player_transform)) = query.get_single_mut() {
        let translation = &player_transform.translation;
        player_velocity.y = if keyboard.pressed(KeyCode::W) {
            if translation.y + 85. < 350. {
                PLAYER_SPEED
            } else {
                0.
            }
        } else if keyboard.pressed(KeyCode::S) {
            if translation.y - 85. > -350. {
                -PLAYER_SPEED
            } else {
                0.
            }
        } else {
            0.
        }
    }
}

fn player_control2(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity2, &Transform), With<Player>>,
) {
    if let Ok((mut velocity, player_transform)) = query.get_single_mut() {
        let translation = &player_transform.translation;
        velocity.y = if keyboard.pressed(KeyCode::Up) {
            if translation.y + 85. < 350. {
                PLAYER_SPEED
            } else {
                0.
            }
        } else if keyboard.pressed(KeyCode::Down) {
            if translation.y - 85. > -350. {
                -PLAYER_SPEED
            } else {
                0.
            }
        } else {
            0.
        }
    }
}

fn cpu_control(
    mut commands: Commands,
    mut aiquery: Query<(&mut VelocityAI, &mut Transform, &mut ReactionBarrier), Without<Ball>>,
    ballquery: Query<(&BallVelocity, &Transform), With<Ball>>,
) {
    for (ball_velocity, ball_tf) in ballquery.iter() {
        let ball_transform = &ball_tf.translation;
        for (mut ai_velocity, mut ai_transform, mut reaction_bar) in aiquery.iter_mut() {
            let translation = &mut ai_transform.translation;
            let reaction_barrier = reaction_bar.x;
            let mut ease = 0.75;
            if ball_velocity.x >= 0. {
                if ball_transform.x >= reaction_barrier {
                    if (translation.y + (ai_velocity.y * 4.) / ease) < ball_transform.y {
                        translation.y += ai_velocity.y / ease;
                        ease += 0.25;
                    } else if (translation.y - (ai_velocity.y * 4.) / ease) > ball_transform.y {
                        translation.y -= ai_velocity.y / ease;
                        ease += 0.75;
                    } else {
                        translation.y += 0.;
                    }
                }
            } else if ball_velocity.x < 0. {
                if translation.y < -30. {
                    translation.y += ai_velocity.y;
                    ease += 0.25;
                } else if translation.y > 30. {
                    translation.y -= ai_velocity.y;
                    ease += 0.25;
                }
            }
            if ball_transform.x >= (900. - (ai_velocity.y + 3.)) {
                if ai_velocity.y < 18. {
                    ai_velocity.y += 5.;
                }
                if reaction_bar.x > -700. {
                    reaction_bar.x -= 70.;
                }
            }
        }
    }
}

fn ball_movement(
    mut commands: Commands,
    mut score: ResMut<Score1>,
    mut score2: ResMut<Score2>,
    mut query: Query<
        (
            Entity,
            &mut BallVelocity,
            &mut Transform,
            &BallMovement,
            &mut SpeedUp,
        ),
        With<Ball>,
    >,
) {
    for (ball_entity, mut ball_velocity, mut ball_transform, ball_movement, mut speedup) in
        query.iter_mut()
    {
        let translation = &mut ball_transform.translation;
        let speedup = &mut speedup.speed;

        translation.y += ball_velocity.y;
        translation.x += ball_velocity.x;

        if ball_movement.auto_despawn {
            if translation.x >= 900. {
                translation.y = 0.;
                translation.x = 0.;
                ball_velocity.y = 0.;
                ball_velocity.x = 5.;
                *speedup = INITAL_SPEED;
                score.score += 1;
            } else if translation.x <= -900. {
                translation.y = 0.;
                translation.x = 0.;
                ball_velocity.y = 0.;
                ball_velocity.x = -5.;
                *speedup = INITAL_SPEED;
                score2.score += 1;
            }
            if translation.y <= -345. {
                ball_velocity.x * 2.;
                ball_velocity.y = (ball_velocity.y * -1.) - 1.;
                translation.y += ball_velocity.y + 5.;
                translation.x += ball_velocity.x;
            } else if translation.y >= 345. {
                ball_velocity.x * 2.;
                ball_velocity.y = (ball_velocity.y * -1.) - 5.;
                translation.y += ball_velocity.y + 5.;
                translation.x += ball_velocity.x;
            }
        }
    }
}

fn ball_collision_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut ball_query: Query<
        (
            Entity,
            &mut BallVelocity,
            &mut Transform,
            &SpriteSize,
            &mut SpeedUp,
        ),
        With<Ball>,
    >,
    player_query: Query<(Entity, &Transform, &SpriteSize), Without<Ball>>,
) {
    for (ball_entity, mut ball_velocity, mut ball_transform, ball_size, mut speedup) in
        ball_query.iter_mut()
    {
        for (player_entity, mut player_transform, player_size) in player_query.iter() {
            let ball_scale = Vec2::from(ball_transform.scale.xy());
            let player_scale = Vec2::from(player_transform.scale.xy());
            let collision = collide(
                ball_transform.translation,
                ball_size.0 * ball_scale,
                player_transform.translation,
                (0., PLAYER_SIZE.1).into(),
            );

            let ball_translation = &mut ball_transform.translation;
            let paddle_translation = &player_transform.translation;
            let speedup = &mut speedup.speed;

            let relative_intersect_y =
                (paddle_translation.y + (PLAYER_SIZE.1 / 2.)) - ball_translation.y;
            let normalized_relative_intersection_y = (relative_intersect_y / (PLAYER_SIZE.1 / 2.));
            let bounce_angle = normalized_relative_intersection_y * MAX_BOUNCE_ANGLE;

            if let Some(_) = collision {
                audio.play(asset_server.load("sounds/Tink.ogg"));
                if *speedup >= MAX_SPEED_UP {
                    *speedup * 1.;
                } else {
                    *speedup += 0.25;
                }
                if ball_translation.x < 0. {
                    if ball_translation.y < paddle_translation.y {
                        ball_velocity.y = ball_velocity.x * bounce_angle.sin();
                    } else if ball_translation.y > paddle_translation.y {
                        ball_velocity.y = -ball_velocity.x * bounce_angle.sin();
                    } else {
                        ball_velocity.y = 0.;
                    }
                    ball_velocity.x = 5. + (*speedup * bounce_angle.cos());
                    ball_translation.y += ball_velocity.y;
                    ball_translation.x += ball_velocity.x;
                } else if ball_translation.x > 0. {
                    if ball_translation.y < paddle_translation.y {
                        ball_velocity.y = -ball_velocity.x * bounce_angle.sin();
                    } else if ball_translation.y > paddle_translation.y {
                        ball_velocity.y = ball_velocity.x * bounce_angle.sin();
                    } else {
                        ball_velocity.y = 0.;
                    }
                    ball_velocity.x = -5. - (*speedup * bounce_angle.cos());
                    ball_translation.y += ball_velocity.y;
                    ball_translation.x += ball_velocity.x;
                }
            }
        }
    }
}

fn update_score1(score: Res<Score1>, mut query: Query<&mut Text, With<Score1>>) {
    let mut text = query.single_mut();
    text.sections[0].value = score.score.to_string();
}
fn update_score2(score: Res<Score2>, mut query: Query<&mut Text, With<Score2>>) {
    let mut text = query.single_mut();
    text.sections[0].value = score.score.to_string();
}

fn play(mut keyboard: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        app_state.pop().unwrap();
        keyboard.reset(KeyCode::Space);
    }
}

fn pause(
    mut commands: Commands,
    mut keyboard: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        app_state.push(AppState::Paused).unwrap();
        keyboard.reset(KeyCode::Space);
    }
}

fn exit_app(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    score1: Res<Score1>,
    score2: Res<Score2>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);

        let result = if score1.score > score2.score {
            "Player1 has won!"
        } else if score1.score == score2.score {
            "Draw"
        } else {
            "Player2 has won!"
        };

        println!("{}", result);
    }
}
