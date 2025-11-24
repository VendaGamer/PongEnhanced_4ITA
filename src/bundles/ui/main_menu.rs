use crate::bundles::{AlignItems, Bundle, FlexDirection, JustifyContent, Val};
use crate::components::ui::{MainMenu, MenuButton};
use bevy::color::Color;
use bevy::prelude::*;
use bevy::prelude::Commands;
use bevy::ui::{BackgroundColor, Node};
use bevy::utils::default;
use crate::bundles::ui::ButtonBundle;
use crate::bundles::ui::container::Container;

#[derive(Bundle)]
pub struct MainMenuBundle {
    menu: MainMenu,
    container: Node,
    background_color: BackgroundColor,
}

impl Default for MainMenuBundle{
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

impl MainMenuBundle {
    fn spawn(commands: &mut Commands)-> Entity{
        commands.spawn(MainMenuBundle::default())
            .with_children(|menuParent| {

                menuParent.spawn(Container::buttons())
                    .with_children(|containerParent|{
                        containerParent.spawn(ButtonBundle::main_menu(Color::srgb(0.5, 0.1, 0.1)));
                        containerParent.spawn(ButtonBundle::main_menu(Color::srgb(0.5, 0.1, 0.1)));
                        containerParent.spawn(ButtonBundle::main_menu(Color::srgb(0.5, 0.1, 0.1)));
                    })


            })
            .id()
    }
}