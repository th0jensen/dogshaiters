use bevy::prelude::*;
use state::{GameState, Score};

mod obstacle;
mod physics;
mod player;
mod poop;
mod setup;
mod state;

#[derive(Resource)]
pub struct SpawnTimer {
    obstacle: Timer,
    poop: Timer,
}

// const PLAYER_SPEED: f32 = 300.0;
pub const JUMP_FORCE: f32 = 500.0;
pub const GRAVITY: f32 = -9.8 * 100.0;
pub const GROUND_Y: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const OBSTACLE_SIZE: f32 = 48.0;
pub const POOP_SIZE: f32 = 32.0;
pub const INITIAL_OBSTACLE_SPEED: f32 = 300.0;
pub const INITIAL_POOP_SPEED: f32 = 175.0;
pub const SPEED_INCREASE_FACTOR: f32 = 1.1;

// Plugin for game logic
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<GameState>()
            .insert_resource(SpawnTimer {
                obstacle: Timer::from_seconds(2.0, TimerMode::Repeating),
                poop: Timer::from_seconds(3.0, TimerMode::Repeating),
            })
            .add_systems(Startup, setup::setup)
            .add_systems(
                Update,
                (
                    player::player_movement,
                    physics::apply_gravity,
                    obstacle::spawn_obstacles,
                    obstacle::obstacle_movement,
                    poop::spawn_poop,
                    poop::collect_poop,
                    poop::poop_movement,
                    physics::check_collision,
                    state::update_score,
                    state::handle_game_over,
                ),
            );
    }
}
