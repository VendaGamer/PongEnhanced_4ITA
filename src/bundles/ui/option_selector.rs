use crate::components::ui::navigation::{OptionSelector, UINavSlot};
use bevy::prelude::*;
use crate::components::ui::effects::HoverLight;

#[derive(Bundle)]
pub struct OptionSelectorBundle {
    pub selector: OptionSelector,
    pub button: Button,
    pub container: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub hover_light: HoverLight,
    pub navigation_slot: UINavSlot,
}

impl OptionSelectorBundle {
    pub fn new(options: Vec<String>, selected: usize, slot: UINavSlot, label: String) -> Self {
        Self {
            selector: OptionSelector { options, selected, label },
            button: Button,
            container: Node {
                width: Val::Px(400.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            hover_light: HoverLight {
                amount: 0.0,
                max: 0.2,
                speed: 3.0,
                base: Color::srgb(0.2, 0.2, 0.25),
            },
            navigation_slot: slot,
        }
    }
}