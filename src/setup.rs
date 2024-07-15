use crate::physics::Collider;
use crate::player::Player;
use crate::state::ScoreText;
use crate::{GROUND_Y, PLAYER_SIZE};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single_mut();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, GROUND_Y, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            ..default()
        },
        Player {
            velocity: Vec2::ZERO,
            alive: true,
        },
    ));

    let ground_size: f32 = 50.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(window.width(), ground_size)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, GROUND_Y - ground_size, 0.0),
            ..default()
        },
        Collider,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/JetBrainsMonoNerdFont-Regular.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));
}
