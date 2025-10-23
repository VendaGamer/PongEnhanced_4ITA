use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bundles::BallBundle;
use crate::components::*;

pub fn handle_scoring(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<Entity, With<Ball>>,
    goal_query: Query<&Goal>,
    mut team_query: Query<&mut Team>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            
            let (ball_entity, goal_entity) =
                if ball_query.contains(*entity1) && goal_query.contains(*entity2) {
                    (entity1, entity2)
                } else if ball_query.contains(*entity2) && goal_query.contains(*entity1) {
                    (entity2, entity1)
                } else {
                    // Not a ball/goal collision
                    continue;
                };

            // It's a goal!
            if let Ok(goal) = goal_query.get(*goal_entity) {
                for mut team in team_query.iter_mut() {
                    if goal.team_id == team.id {
                        team.current_score += 1;
                    }
                }

                // Reset ball
                // Despawn old ball
                commands.entity(*ball_entity).despawn();
                // Spawn new ball
                commands.spawn(BallBundle::new(
                    &mut meshes,
                    &mut materials,
                    Vec3::ZERO,
                    Vec2::new(100.0, 0.0), // Send it to the other player
                ));
            }
        }
    }
}