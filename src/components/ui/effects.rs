use crate::bundles::{Color, Component};

#[derive(Component)]
pub struct HoverLight {
    pub amount: f32,
    pub max: f32,
    pub speed: f32,
    pub base: Color,
}