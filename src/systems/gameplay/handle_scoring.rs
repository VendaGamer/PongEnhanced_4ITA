use crate::bundles::BallBundle;
use crate::components::*;
use crate::utils::screen::BALL_RADIUS;
use avian2d::prelude::*;
use bevy::prelude::*;
use crate::resources::GameConfig;

pub fn handle_scoring(
    collision: On<CollisionStart>,
    goals: Query<(Entity, &Goal)>,
    mut game_config: ResMut<GameConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let ball = collision.collider1;
    let other = collision.collider2;

    if let Ok(goal) = goals.get(other){
        if let Some(team) = game_config.area_shape.get_team_mut(goal.0) {
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