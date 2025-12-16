use crate::bundles::BallBundle;
use crate::components::ui::ScoreText;
use crate::components::*;
use crate::resources::GameConfig;
use crate::utils::screen::BALL_RADIUS;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn handle_scoring(
    collision: On<CollisionStart>,
    goals: Query<&Goal>,
    mut game_config: ResMut<GameConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let ball = collision.collider1;
    let other = collision.collider2;

    if let Ok(goal) = goals.get(other) {
        if let Some(team) = game_config.area_shape.get_team_mut(goal.side){

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

pub fn update_score_ui(
    mut game_config: ResMut<GameConfig>,
    mut score_texts: Query<(&mut Text, &ScoreText)>,
) {
    if !game_config.is_changed(){
        return;
    }

    for (mut text, score_text) in score_texts.iter_mut() {
        if let Some(team) = game_config.area_shape.get_team(score_text.area_side){
            text.0 = team.current_score.to_string();
        }
    }
}