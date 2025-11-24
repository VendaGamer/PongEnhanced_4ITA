use bevy::color::Color;
use bevy::image::DataFormat::Rgba;
use bevy::prelude::children;
use bevy::ui::{BackgroundColor, Node};
use bevy::utils::default;
use crate::bundles::{AlignItems, Bundle, ColorMaterial, FlexDirection, JustifyContent, Mesh2d, MeshMaterial2d, Transform, Val};
use crate::components::DivisionLine;
use crate::components::ui::MainMenu;

#[derive(Bundle)]
pub struct MainMenuBundle {
    menu: MainMenu,
    container: Node,
    background_color: BackgroundColor,
}

#[derive(Bundle)]
struct ButtonsContainer{

}

impl MainMenuBundle {
    fn new()-> Self{
        MainMenuBundle{
            menu: MainMenu,
            container: Node{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.05, 0.05, 0.1))
        }
    }
}