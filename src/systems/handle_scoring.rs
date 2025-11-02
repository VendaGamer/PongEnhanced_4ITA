use bevy::prelude::*;
use avian2d::prelude::*;
use crate::bundles::BallBundle;
use crate::components::*;

pub fn handle_scoring(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    ball_query: Query<Entity, With<Ball>>,
    goal_query: Query<&Goal>,
    mut team_query: Query<&mut Team>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (contacts) in collision_events.read() {
        let entity1 = contacts.collider1;
        let entity2 = contacts.collider2;

        let (ball_entity, goal_entity) =
            if ball_query.contains(entity1) && goal_query.contains(entity2) {
                (entity1, entity2)
            } else if ball_query.contains(entity2) && goal_query.contains(entity1) {
                (entity2, entity1)
            } else {
                // Not a ball/goal collision
                continue;
            };

        // It's a goal!
        if let Ok(goal) = goal_query.get(goal_entity) {
            for mut team in team_query.iter_mut() {
                if goal.team_id == team.id {
                    team.current_score += 1;
                }
            }

            // Reset ball
            commands.entity(ball_entity).despawn();
            commands.spawn(BallBundle::new(
                &mut meshes,
                &mut materials,
                Vec3::ZERO,
                Vec2::new(100.0, 0.0),
            ));
        }
    }
}