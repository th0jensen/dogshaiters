use crate::state::GameState;
use crate::{state::Score, SpawnTimer};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

#[derive(Component)]
pub struct Obstacle {
    pub velocity: Vec2,
}

pub fn spawn_obstacles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    game_state: Res<GameState>,
    score: Res<Score>,
) {
    if let GameState::Playing = *game_state {
        let mut rng = rand::thread_rng();

        // Decrease spawn timer as score increases
        let spawn_time = (2.0 / (1.0 + score.x as f32 / 500.0)).max(0.5);
        spawn_timer
            .obstacle
            .set_duration(std::time::Duration::from_secs_f32(spawn_time));

        if spawn_timer.obstacle.tick(time.delta()).just_finished() {
            let window = windows.single();
            let speed = crate::INITIAL_OBSTACLE_SPEED
                * crate::SPEED_INCREASE_FACTOR.powf(score.x as f32 / 100.0);

            // Add some randomness to the speed
            let random_speed = speed * rng.gen_range(0.75..1.25);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("obstacle.png"),
                    transform: Transform::from_xyz(window.width(), crate::GROUND_Y, 1.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(crate::OBSTACLE_SIZE, crate::OBSTACLE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                Obstacle {
                    velocity: Vec2::new(-random_speed, 0.0),
                },
            ));
        }
    }
}

pub fn obstacle_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Obstacle), With<Obstacle>>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();

    for (entity, mut transform, obstacle) in query.iter_mut() {
        transform.translation += obstacle.velocity.extend(0.0) * time.delta_seconds();

        if transform.translation.x < -window.width() / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
