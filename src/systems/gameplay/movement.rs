use crate::components::*;
use crate::resources::controls::*;
use crate::utils::screen::PADDLE_SIZE;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const BALL_SPEED: f32 = 600.0;

pub fn move_paddle(
    time: Res<Time>,
    player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
) {
    for (mut paddle_transform, paddle) in paddle_query.iter_mut() {
        if let Ok(action_state) = player_query.get(paddle.player) {
            let mut move_amount = time.delta_secs() * 400.0;
            
            if action_state.pressed(&PlayerAction::Speedup) {
                move_amount *= 2.0;
            }

            if action_state.pressed(&PlayerAction::Up){
                paddle_transform.translation.y += move_amount;
            }
            
            if action_state.pressed(&PlayerAction::Down) {
                paddle_transform.translation.y -= move_amount;
            }
        }
    }
}

pub fn maintain_ball_speed(
    mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
) {


    for mut velocity in ball_query.iter_mut() {
        let current_speed = velocity.length();
        if current_speed > 0.0 {
            velocity.0 = velocity.normalize() * BALL_SPEED;
        }
    }
}

pub fn paddle_hit_dynamics(
    mut collision_events: MessageReader<CollisionStart>,
    mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    ball_transform_query: Query<&Transform, With<Ball>>,
) {
    for (contacts) in collision_events.read() {
        let entity1 = contacts.collider1;
        let entity2 = contacts.collider2;

        // Determine which is ball and which is paddle
        let (ball_entity, paddle_entity) =
            if ball_query.contains(entity1) && paddle_query.contains(entity2) {
                (entity1, entity2)
            } else if ball_query.contains(entity2) && paddle_query.contains(entity1) {
                (entity2, entity1)
            } else {
                continue;
            };

        if let (Ok(mut ball_vel), Ok(paddle_transform), Ok(ball_transform)) =
            (ball_query.get_mut(ball_entity),
             paddle_query.get(paddle_entity),
             ball_transform_query.get(ball_entity)) {

            // Calculate hit offset from paddle center (-1.0 to 1.0)
            let paddle_half_height = PADDLE_SIZE.y/2.0;
            let offset = (ball_transform.translation.y - paddle_transform.translation.y)
                / paddle_half_height;
            
            let speed = ball_vel.length();
            let new_y_vel = offset * speed * 0.75;

            ball_vel.y = new_y_vel;
            // Maintain speed
            ball_vel.0 = ball_vel.normalize() * speed;
        }
    }
}