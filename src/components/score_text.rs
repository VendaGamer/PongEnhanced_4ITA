use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct ScoreText {
    pub team: Entity,
}