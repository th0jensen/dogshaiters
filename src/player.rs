use crate::{GROUND_Y, JUMP_FORCE};
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub alive: bool,
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut touch_events: EventReader<TouchInput>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        let keyboard_jump = keyboard_input.pressed(KeyCode::Space);
        let touch_jump = touch_events
            .read()
            .any(|touch| touch.phase == TouchPhase::Started);

        if (keyboard_jump || touch_jump) && transform.translation.y <= GROUND_Y {
            player.velocity.y = JUMP_FORCE;
        }

        transform.translation.y += player.velocity.y * time.delta_seconds();
        transform.translation.y = transform.translation.y.max(GROUND_Y);
    }
}
