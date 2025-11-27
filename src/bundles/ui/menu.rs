use crate::components::ui::*;
use bevy::prelude::*;
use crate::bundles::ui::widgets::{ButtonBundle, LabelBundle};

#[derive(Bundle)]
pub struct MenuBundle {
    menu: MainMenu,
    container: Node,
    background_color: BackgroundColor,
}

impl Default for MenuBundle {
    fn default() -> Self {
        Self{
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

impl MenuBundle {
    pub fn spawn_main_menu(commands: &mut Commands) {

        commands.spawn(MenuBundle::default())
            .with_children(|parent|{
                parent.spawn(LabelBundle::game_title());
                ButtonBundle::spawn_main_menu_buttons(parent);
            });

    }
}