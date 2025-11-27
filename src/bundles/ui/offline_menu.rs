use crate::bundles::menu_section::MenuSectionBundle;
use crate::bundles::option_selector::OptionSelectorBundle;
use crate::bundles::ui::*;
use crate::components::ui::navigation::{SelectorText, UINavSlot};
use crate::components::ui::*;
use crate::systems::ButtonPressed;
use bevy::prelude::*;
use crate::bundles::ui::widgets::ButtonBundle;

#[derive(Bundle)]
pub struct OfflinePlayMenuBundle {
    pub offline_menu: OfflinePlayMenu,
    pub container: Node,
    pub background_color: BackgroundColor,
}

impl Default for OfflinePlayMenuBundle {
    fn default() -> Self {
        Self {
            offline_menu: OfflinePlayMenu,
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

impl OfflinePlayMenuBundle {
    pub fn spawn_offline_menu(commands: &mut Commands) {
        commands.spawn(OfflinePlayMenuBundle::default())
            .with_children(|parent| {
                // Title
                parent.spawn((
                    Node {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    },
                    Text::new("OFFLINE PLAY"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                ));

                // Game options container
                parent.spawn(MenuSectionBundle::new())
                    .with_children(|section| {
                        section.spawn((
                            Node {
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            },
                            Text::new("Game Setup"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.9)),
                        ));

                        // Number of Players
                        section.spawn(OptionSelectorBundle::new(
                            vec!["2 Players".into(), "3 Players".into(), "4 Players".into()],
                            0,
                            UINavSlot::new(0, 0),
                            "Number of Players".into(),
                        )).with_children(|sel| {
                            sel.spawn((
                                Text::new("Players: 2 Players"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::WHITE),
                                SelectorText { selector_entity: sel.target_entity() },
                            ));
                        });

                        // Game Mode
                        section.spawn(OptionSelectorBundle::new(
                            vec![
                                "Classic".into(),
                                "Modern".into(),
                                "Upside Down".into(),
                                "Black Out".into(),
                                "Twisted".into(),
                            ],
                            0,
                            UINavSlot::new(1, 0),
                            "Game Mode".into(),
                        )).with_children(|sel| {
                            sel.spawn((
                                Text::new("Mode: Classic"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::WHITE),
                                SelectorText { selector_entity: sel.target_entity() },
                            ));
                        });

                        // Arena Shape
                        section.spawn(OptionSelectorBundle::new(
                            vec!["Two Side".into(), "Triangular".into(), "Cuboid".into()],
                            0,
                            UINavSlot::new(2, 0),
                            "Arena Shape".into(),
                        )).with_children(|sel| {
                            sel.spawn((
                                Text::new("Arena: Two Side"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::WHITE),
                                SelectorText { selector_entity: sel.target_entity() },
                            ));
                        });


                        // Win Score
                        section.spawn(OptionSelectorBundle::new(
                            vec!["5 Points".into(), "10 Points".into(), "15 Points".into(), "20 Points".into()],
                            1,
                            UINavSlot::new(3, 0),
                            "Win Score".into(),
                        )).with_children(|sel| {
                            sel.spawn((
                                Text::new("Score: 10 Points"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::WHITE),
                                SelectorText { selector_entity: sel.target_entity() },
                            ));
                        });

                    });

                // Button row
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::top(Val::Px(30.0)),
                    column_gap: Val::Px(20.0),
                    ..default()
                }).with_children(|buttons| {
                    buttons.spawn(ButtonBundle::menu_button(
                        Color::srgb(0.2, 0.7, 0.3),
                        "Start Game",
                        UINavSlot::new(4, 0),
                    )).observe(on_start_offline_game);

                    buttons.spawn(ButtonBundle::menu_button(
                        Color::srgb(0.6, 0.6, 0.6),
                        "Back",
                        UINavSlot::new(4, 1),
                    )).observe(on_offline_back);
                });
            });
    }
}

fn on_start_offline_game(
    _press: On<ButtonPressed>,
    mut commands: Commands,
) {
    
}

fn on_offline_back(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    offline_menu: Query<Entity, With<OfflinePlayMenu>>,
) {
    for entity in &offline_menu {
        commands.entity(entity).despawn();
    }
    MenuBundle::spawn_main_menu(&mut commands);
}