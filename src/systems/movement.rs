use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::components::*;
use crate::resources::controls::*;

pub fn move_paddle(
    time: Res<Time>,
    player_query: Query<(&ActionState<PlayerAction>, &ControlledPaddle), With<Player>>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    let move_amount = time.delta_secs() * 400.0;

    for (action_state, controlled_paddle) in player_query.iter() {
        let paddle_entity = controlled_paddle.0;

        if let Ok(mut paddle_transform) = paddle_query.get_mut(paddle_entity) {

            if action_state.pressed(&PlayerAction::Up) {
                paddle_transform.translation.y += move_amount;
            }
            if action_state.pressed(&PlayerAction::Down) {
                paddle_transform.translation.y -= move_amount;
            }
        }
    }
}