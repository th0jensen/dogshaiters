use bevy::input::touch::TouchPhase;
use bevy::prelude::*;

use crate::player::Player;
use crate::{GROUND_Y, PLAYER_SIZE};

#[derive(Resource, Default)]
pub struct Score {
    pub x: u32,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Resource, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", score.x);
}

pub fn handle_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut touch_events: EventReader<TouchInput>,
    mut game_state: ResMut<GameState>,
    mut score: ResMut<Score>,
) {
    if let GameState::GameOver = *game_state {
        let keyboard_return = keyboard_input.just_pressed(KeyCode::Enter);
        let touch_return = touch_events
            .read()
            .any(|touch| touch.phase == TouchPhase::Started);
        if keyboard_return || touch_return {
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

            score.x = 0;
            *game_state = GameState::Playing;
        }
    }
}
