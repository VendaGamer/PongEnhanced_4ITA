use bevy::input::keyboard::Key::ScreenModeNext;
use bevy::prelude::*;
use bevy::window::WindowMode::BorderlessFullscreen;
use crate::components::ui::*;
use crate::bundles::*;
use crate::bundles::ui::menu_section::MenuSectionBundle;
use crate::bundles::ui::widgets::ButtonBundle;
use crate::bundles::widgets::{OptionSelectorBundle, SliderBundle};
use crate::components::GameMode;
use crate::components::ui::navigation::{SelectorText, UINavSlot};
use crate::models::game::fullscreen::ScreenMode;
use crate::models::ui::option::UIOption;
use crate::systems::ButtonPressed;

#[derive(Bundle)]
pub struct SettingsMenuBundle {
    pub settings_menu: SettingsMenu,
    pub container: Node,
    pub background_color: BackgroundColor,
}

impl Default for SettingsMenuBundle {
    fn default() -> Self {
        Self {
            settings_menu: SettingsMenu,
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

impl SettingsMenuBundle {
    pub fn spawn_settings_menu(commands: &mut Commands) {
        commands.spawn(SettingsMenuBundle::default())
            .with_children(|parent| {
                // Title
                parent.spawn((
                    Node {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    },
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                ));

                // Settings sections container
                parent.spawn(MenuSectionBundle::new())
                    .with_children(|section| {
                        // Audio Section Header
                        section.spawn((
                            Node {
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            },
                            Text::new("Audio"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.9)),
                        ));

                        // Master Volume selector
                        section.spawn(SliderBundle::new(0.0, 100.0, 0.0, UINavSlot::row(0)));


                        // SFX Volume selector
                        section.spawn(SliderBundle::new(0.0, 100.0, 0.0, UINavSlot::row(1)));

                        // Graphics Section Header
                        section.spawn((
                            Node {
                                margin: UiRect::vertical(Val::Px(20.0)),
                                ..default()
                            },
                            Text::new("Graphics"),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.9)),
                        ));

                        // Resolution selector
                        section.spawn(OptionSelectorBundle::new(
                            vec![
                                UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),

                            ],
                            0,
                            UINavSlot::new(2, 0),
                            "Resolution".into(),
                        )).with_children(|sel| {
                            sel.spawn((
                                Text::new("Resolution: 1280x720"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::WHITE),
                                SelectorText { selector_entity: sel.target_entity() },
                            ));
                        });;

                        // Fullscreen selector
                        section.spawn(OptionSelectorBundle::new(
                    vec![
                        UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                        UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                        UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                                ],
                        0,
                            UINavSlot::new(3, 0),
                            "Fullscreen".into(),
                        )).with_children(|sel| {
                            sel.spawn((
                                Text::new("Fullscreen: Off"),
                                TextFont { font_size: 24.0, ..default() },
                                TextColor(Color::WHITE),
                                SelectorText { selector_entity: sel.target_entity() },
                            ));
                        });
                    });

                // Back button
                parent.spawn(
                    ButtonBundle::menu_button(
                        Color::srgb(0.6, 0.6, 0.6),
                        "Back",
                        UINavSlot::row(4),
                )).observe(on_settings_back);
            });
    }
}

fn on_settings_back(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    settings_menu: Query<Entity, With<SettingsMenu>>,
) {
    for entity in &settings_menu {
        commands.entity(entity).despawn();
    }
    MenuBundle::spawn_main_menu(&mut commands);
}