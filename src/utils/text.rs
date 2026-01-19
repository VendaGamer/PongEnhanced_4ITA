use bevy::prelude::*;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../../assets/font/jersey10_regular.ttf");
pub const DEFAULT_LIGHTEN_AMOUNT: f32 = 30.0;
pub const PIXEL_BORDER_SIZE: f32 = 3.0;


#[derive(Clone, Copy)]
pub struct RetroTheme {
    pub button: Color,
    pub button_hover: Color,
    pub button_pressed: Color,
    pub slider_track: Color,
    pub slider_thumb: Color,
    pub border: Color,
    pub border_dark: Color,
    pub outline: Color,
    pub accent: Color,
    pub panel_bg: Color,
    pub section_bg: Color,
    pub text_bright: Color,
    pub text_normal: Color,
}

pub const MODERN_THEME: RetroTheme = RetroTheme {
    button: Color::srgb(0.15, 0.15, 0.15),
    button_hover: Color::srgb(0.25, 0.25, 0.25),
    button_pressed: Color::srgb(0.35, 0.75, 0.35),
    slider_track: Color::srgb(0.05, 0.05, 0.05),
    slider_thumb: Color::srgb(0.35, 0.75, 0.35),
    border: Color::srgb(0.45, 0.45, 0.45),
    border_dark: Color::srgb(0.3, 0.3, 0.3),
    outline: Color::srgb(0.45, 0.45, 0.45),
    accent: Color::srgb(0.35, 0.75, 0.35),
    panel_bg: Color::srgb(0.2, 0.2, 0.25),
    section_bg: Color::srgb(0.1, 0.1, 0.1),
    text_bright: Color::srgb(0.9, 0.9, 1.0),
    text_normal: Color::srgb(0.8, 0.8, 0.8),
};

pub fn lighten_color(color: Color, amount: f32) -> Color {
    let [r, g, b, a] = color.to_srgba().to_f32_array();
    Color::srgba(
        (r + amount / 100.0).min(1.0),
        (g + amount / 100.0).min(1.0),
        (b + amount / 100.0).min(1.0),
        a,
    )
}