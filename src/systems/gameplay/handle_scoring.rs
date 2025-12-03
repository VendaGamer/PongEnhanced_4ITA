use crate::bundles::BallBundle;
use crate::components::*;
use crate::utils::screen::BALL_RADIUS;
use avian2d::prelude::*;
use bevy::prelude::*;
use crate::resources::GameConfig;

pub fn handle_scoring(
    collision: On<CollisionStart>,
    goals: Query<&Goal>,
    game_config: Res<GameConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let ball = collision.collider1;
    let other = collision.collider2;

    if let Ok(goal) = goals.get(other){
        game_config.area_shape
        if let Ok(mut team) = teams.get_mut(goal.team) {
            team.current_score += 1;

            commands.entity(ball).despawn();

            commands.spawn(BallBundle::new(
                &mut meshes,
                &mut materials,
                Vec3::ZERO,
                Vec2::new(-300.0, 300.0),
                BALL_RADIUS
            )).observe(handle_scoring);
        }
    }
}