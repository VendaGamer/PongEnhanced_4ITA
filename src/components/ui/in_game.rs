use crate::bundles::{Component, Entity};

#[derive(Component)]
pub struct ScoreText {
    pub team: Entity,
}