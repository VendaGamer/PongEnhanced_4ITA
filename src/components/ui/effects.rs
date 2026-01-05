use crate::bundles::{Color, Component};
use crate::utils::lighten_color;

#[derive(Component)]
pub struct HoverLight;

#[derive(Component)]
pub struct HoverLightColor {
    pub hover_color: Color,
}


impl HoverLightColor {
    pub fn new(base_color: Color, lighten_amount: f32) -> Self {
        Self {
            hover_color: lighten_color(base_color, lighten_amount),
        }
    }
}