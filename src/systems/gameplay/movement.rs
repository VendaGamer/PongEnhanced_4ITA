use std::time::Duration;
use crate::components::*;
use crate::events::gameplay::{BallBounced, GoalScored};
use crate::resources::controls::*;
use crate::utils::screen::PADDLE_SIZE;
use crate::utils::HALF_HEIGHT;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_tween::interpolate::rotation_delta_by;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::AnimationBuilderExt;
use bevy_tween::tween::IntoTarget;
use leafwing_input_manager::prelude::*;
use crate::components::game_modes::{PaddleTilt, MAX_ABS_TILT};

const BALL_SPEED: f32 = 600.0;

pub fn u_move_paddle_i(
    player_query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
    time: Res<Time>
) {
    for (mut transform, paddle) in paddle_query.iter_mut() {

        for (action_state, player) in player_query {
            if player.id.eq(&paddle.id) {
                if let Some(data) = action_state.axis_data(&PlayerAction::Move) {
                    transform.translation.y += 600.0 * time.delta_secs() * data.update_value;

                    let half_paddle_height = PADDLE_SIZE.y / 2.0;
                    let limit = HALF_HEIGHT - half_paddle_height;
                    transform.translation.y = transform.translation.y.clamp(-limit, limit);
                }
                break;
            }
        }
    }
}

pub fn u_tilt_i(
    player_query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut paddle_query: Query<(Entity, &mut Transform, &Paddle)>,
    mut commands: Commands
) {
    for (paddle_e, mut tilt, paddle) in paddle_query.iter_mut() {

        for (action_state, player) in player_query {
            if player.id.eq(&paddle.id) {
                    if let Some(data) = action_state.axis_data(&PlayerAction::Tilt){

                        let target = paddle_e.into_target();

                        commands.animation().insert_tween_here(
                            Duration::from_secs_f32(0.3),
                            EaseKind::CubicInOut,
                            target.state(Quat::from_rotation_z(0.0)).with(rotation_delta_by(
                                Quat::from_rotation_z(data.value.signum() * MAX_ABS_TILT))
                            )
                        );

                    }

                }
            break;
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
    bounce: On<BallBounced>,
    mut ball_query: Query<(&mut LinearVelocity, &Transform), With<Ball>>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    if let (Ok((mut ball_vel, ball_transform)), Ok(paddle_transform))
        = (ball_query.get_mut(bounce.ball), paddle_query.get(bounce.paddle)) {

        let paddle_half_height = PADDLE_SIZE.y/2.0;
        let offset = (ball_transform.translation.y - paddle_transform.translation.y) / paddle_half_height;

        let speed = ball_vel.length();
        let new_y_vel = offset * speed * 0.75;

        ball_vel.y = new_y_vel;
        ball_vel.0 = ball_vel.normalize() * speed;
    }
}

pub fn t_ball_events(
    collision: On<CollisionStart>,
    query: Query<(Option<&Paddle>, Option<&Goal>), With<Collider>>,
    mut commands: Commands
) {
    if let Ok((paddle, goal)) = query.get(collision.collider2) {
        if paddle.is_some() {
            commands.trigger(BallBounced {
                paddle: collision.collider2,
                ball: collision.collider1
            });
        } else if goal.is_some() {
            commands.trigger(GoalScored {
                goal: collision.collider2,
                ball: collision.collider1
            })
        }
    }
}