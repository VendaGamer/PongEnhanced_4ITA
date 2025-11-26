use bevy::prelude::ColorToComponents;
use crate::bundles::Color;

pub fn lighten_color(color: Color, amount: f32) -> Color {
    let [r, g, b, a] = color.to_srgba().to_f32_array();
    Color::srgba(
        (r + amount).min(1.0),
        (g + amount).min(1.0),
        (b + amount).min(1.0),
        a,
    )
}