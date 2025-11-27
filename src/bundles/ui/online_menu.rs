use crate::bundles::ui::*;
use crate::components::ui::*;
use crate::systems::ButtonPressed;
use bevy::prelude::*;
use crate::bundles::menu_section::MenuSectionBundle;
use crate::bundles::ui::widgets::ButtonBundle;
use crate::components::ui::navigation::UINavSlot;

#[derive(Bundle)]
pub struct OnlinePlayMenuBundle {
    pub online_menu: OnlinePlayMenu,
    pub container: Node,
    pub background_color: BackgroundColor,
}

impl Default for OnlinePlayMenuBundle {
    fn default() -> Self {
        Self {
            online_menu: OnlinePlayMenu,
            container: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
        }
    }
}

impl OnlinePlayMenuBundle {
    pub fn spawn_online_menu(commands: &mut Commands) {
        commands.spawn(OnlinePlayMenuBundle::default())
            .with_children(|parent| {
                // Title
                parent.spawn((
                    Node {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    },
                    Text::new("ONLINE PLAY"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                ));

                // Connection status
                parent.spawn((
                    Node {
                        margin: UiRect::bottom(Val::Px(30.0)),
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                    BorderRadius::all(Val::Px(8.0)),
                    Text::new("Status: Not Connected"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.7, 0.3)),
                ));

                // Online options container
                parent.spawn(MenuSectionBundle::new())
                    .with_children(|section| {
                        section.spawn(ButtonBundle::menu_button(
                            Color::srgb(0.3, 0.6, 0.9),
                            "Quick Match",
                            UINavSlot::row(0),
                        )).observe(on_quick_match);

                        section.spawn(ButtonBundle::menu_button(
                            Color::srgb(0.5, 0.4, 0.9),
                            "Create Room",
                            UINavSlot::row(1),
                        )).observe(on_create_room);

                        section.spawn(ButtonBundle::menu_button(
                            Color::srgb(0.9, 0.5, 0.3),
                            "Join Room",
                            UINavSlot::row(2),
                        )).observe(on_join_room);

                        section.spawn(ButtonBundle::menu_button(
                            Color::srgb(0.4, 0.7, 0.4),
                            "Friends List",
                            UINavSlot::row(3),
                        )).observe(on_friends_list);
                    });

                // Back button
                parent.spawn(ButtonBundle::menu_button(
                    Color::srgb(0.6, 0.6, 0.6),
                    "Back",
                    UINavSlot::row(4),
                )).observe(on_online_back);
            });
    }
}

fn on_quick_match(_press: On<ButtonPressed>) {
    println!("Searching for quick match...");
}

fn on_create_room(_press: On<ButtonPressed>) {
    println!("Creating room...");
}

fn on_join_room(_press: On<ButtonPressed>) {
    println!("Join room menu...");
}

fn on_friends_list(_press: On<ButtonPressed>) {
    println!("Opening friends list...");
}

fn on_online_back(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    online_menu: Query<Entity, With<OnlinePlayMenu>>,
) {
    for entity in &online_menu {
        commands.entity(entity).despawn();
    }
    MenuBundle::spawn_main_menu(&mut commands);
}