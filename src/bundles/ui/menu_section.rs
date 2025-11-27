use bevy::prelude::*;

#[derive(Bundle)]
pub struct MenuSectionBundle {
    pub container: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl MenuSectionBundle {
    pub fn new() -> Self {
        Self {
            container: Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(30.0)),
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            border_radius: BorderRadius::all(Val::Px(10.0)),
        }
    }
}