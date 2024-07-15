use crate::{
    player::Player, state::GameState, Score, SpawnTimer, GROUND_Y, INITIAL_POOP_SPEED, PLAYER_SIZE,
    POOP_SIZE, SPEED_INCREASE_FACTOR,
};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

#[derive(Component)]
pub struct Poop {
    pub velocity: Vec2,
}

pub fn spawn_poop(
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
        let spawn_time = (3.0 / (1.0 + score.x as f32 / 400.0)).max(0.7);
        spawn_timer
            .poop
            .set_duration(std::time::Duration::from_secs_f32(spawn_time));

        if spawn_timer.poop.tick(time.delta()).just_finished() {
            let window = windows.single();
            let speed = INITIAL_POOP_SPEED * SPEED_INCREASE_FACTOR.powf(score.x as f32 / 100.0);

            // Add some randomness to the speed
            let random_speed = speed * rng.gen_range(0.75..1.25);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("poop.png"),
                    transform: Transform::from_xyz(window.width(), GROUND_Y, 1.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(POOP_SIZE, POOP_SIZE / 0.75)),
                        ..default()
                    },
                    ..default()
                },
                Poop {
                    velocity: Vec2::new(-random_speed, 0.0),
                },
            ));
        }
    }
}

pub fn collect_poop(
    mut commands: Commands,
    player_query: Query<(&Transform, &Player)>,
    poop_query: Query<(Entity, &Transform), With<Poop>>,
    mut score: ResMut<Score>,
) {
    if let Ok((player_transform, player)) = player_query.get_single() {
        // Check if the player is alive
        if player.alive {
            for (poop_entity, poop_transform) in poop_query.iter() {
                if player_transform
                    .translation
                    .distance(poop_transform.translation)
                    < (PLAYER_SIZE + POOP_SIZE) / 2.0
                {
                    commands.entity(poop_entity).despawn();
                    score.x += 10;
                }
            }
        }
    }
}

pub fn poop_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Poop)>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();

    for (entity, mut transform, poop) in query.iter_mut() {
        transform.translation += poop.velocity.extend(0.0) * time.delta_seconds();

        if transform.translation.x < -window.width() / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
