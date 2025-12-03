use crate::bundles::{Component, Entity};
use crate::components::Goal;

#[derive(Component)]
pub struct ScoreText {
    pub goal: Entity,
}