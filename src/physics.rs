use crate::obstacle::Obstacle;
use crate::player::Player;
use crate::poop::Poop;
use crate::state::GameState;
use crate::{GRAVITY, GROUND_Y, OBSTACLE_SIZE, PLAYER_SIZE};
use bevy::prelude::*;

#[derive(Component)]
struct _Ground;

#[derive(Component)]
pub struct Collider;

pub fn apply_gravity(mut query: Query<(&mut Player, &Transform)>, time: Res<Time>) {
    if let Ok((mut player, transform)) = query.get_single_mut() {
        if transform.translation.y <= GROUND_Y {
            player.velocity.y = 0.0;
        } else {
            player.velocity.y += GRAVITY * time.delta_seconds();
        }
    }
}

pub fn check_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Sprite, &mut Player)>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    mut game_state: ResMut<GameState>,
    del_player_query: Query<Entity, With<Player>>,
    del_obstacle_query: Query<Entity, With<Obstacle>>,
    del_poop_query: Query<Entity, With<Poop>>,
) {
    if let Ok((player_transform, mut player_sprite, mut player)) = player_query.get_single_mut() {
        for obstacle_transform in obstacle_query.iter() {
            if player_transform
                .translation
                .distance(obstacle_transform.translation)
                < (PLAYER_SIZE + OBSTACLE_SIZE) / 2.0
            {
                player.alive = false;
                player_sprite.color = Color::srgb(1.0, 0.0, 0.0);
                *game_state = GameState::GameOver;
                for entity in del_player_query
                    .iter()
                    .chain(del_obstacle_query.iter())
                    .chain(del_poop_query.iter())
                {
                    commands.entity(entity).despawn();
                }
                break;
            }
        }
    }
}
