use crate::components::Player;
use bevy::prelude::KeyCode::*;
use bevy::prelude::*;

pub fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    let move_amount = time.delta_secs() * 200.0;
    
    if input.pressed(KeyW) {
        for mut transform in &mut query {
            transform.translation.y += move_amount;
        }
    }else if input.pressed(KeyS) {
        for mut transform in &mut query {
            transform.translation.y -= move_amount;
        }
    }
}