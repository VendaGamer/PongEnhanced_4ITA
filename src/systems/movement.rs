use bevy::prelude::*;
use bevy::prelude::KeyCode::*;
use bevy_rapier2d::dynamics::Velocity;
use crate::Ball;
use crate::components::Player;

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

pub fn move_ball(
    mut query: Query<&mut Velocity, With<Ball>>
){
    for mut ball in &mut query {
        ball.linvel += Vec2::new(-0.1, 0.0);
    }
}