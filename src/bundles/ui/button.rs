use crate::bundles::{default, AlignItems, BackgroundColor, BorderColor, BorderRadius, Bundle, Color, JustifyContent, Node, Val};
use bevy::prelude::{Alpha, Button};

#[derive(Bundle)]
pub struct ButtonBundle {
    button: Button,
    container: Node,
    background_color: BackgroundColor,
    border_radius: BorderRadius,
    border_color: BorderColor,
}

impl ButtonBundle{
    pub fn main_menu(color: Color) -> Self{
        Self{
            button: Button,
            container:
            Node {
                width: Val::Px(350.0),
                height: Val::Px(70.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(color),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            border_color: BorderColor::from(Color::WHITE.with_alpha(0.3)),
        }
    }
}