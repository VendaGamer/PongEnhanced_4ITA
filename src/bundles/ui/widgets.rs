use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::ui_widgets::{Slider, SliderRange, SliderValue, TrackClick};
use crate::bundles::{children, default, BackgroundColor, BorderRadius, Bundle, ChildOf, Color, Node, Spawn, Text, TextColor, TextFont, UiRect, Val};
use crate::bundles::ui::main_menu::*;
use crate::components::ui::Dropdown;
use crate::components::ui::effects::HoverLight;
use crate::components::ui::navigation::UINavSlot;

#[derive(Bundle)]
pub struct SliderBundle {
    pub slider: Slider,
    pub slider_range: SliderRange,
    pub slider_value: SliderValue,
    pub container: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub hover_light: HoverLight,
    pub navigation_slot: UINavSlot,
}

impl SliderBundle {
    pub fn new(min: f32, max: f32, current: f32, slot: UINavSlot) -> Self {
        Self {
            slider: Slider {
                track_click: TrackClick::Step
            },
            slider_range: SliderRange::new(min, max),
            slider_value: SliderValue(current),
            container: Node {
                width: Val::Px(300.0),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(4.0)),
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


const GAME_TITLE: &str = "PONG ENHANCED";

#[derive(Bundle)]
pub struct LabelBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}
pub type MainMenuLabel = (Node, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);

impl LabelBundle {
    pub fn button_label(text: &str) -> Self{
        Self{
            text: Text::new(text),
            font: TextFont {
                font_size: 32.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
        }
    }

    pub fn game_title() -> MainMenuLabel {
        (
            Node{
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
            children![
                Self{
                    text: Text::new(GAME_TITLE),
                    font: TextFont {
                        font_size: 72.0,
                        ..default()
                    },
                    color: TextColor(Color::srgb(0.9, 0.9, 1.0)),
                }]
        )
    }
}

#[derive(Bundle)]
pub struct DropdownBundle<T>
    where T: Copy + 'static + Send + Sync,
{
    pub dropdown: Dropdown<T>,
    pub container: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub hover_light: HoverLight,
    pub navigation_slot: UINavSlot,
}

impl<T> DropdownBundle<T>
    where T: Copy + 'static + Send + Sync,
{
    pub fn new(options: Vec<Option<T>>, selected: usize, slot: UINavSlot) -> Self {
        Self {
            dropdown: Dropdown { options, selected },
            container: Node {
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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


#[derive(Bundle)]
pub struct ButtonBundle {
    button: Button,
    container: Node,
    background_color: BackgroundColor,
    border_radius: BorderRadius,
    border_color: BorderColor,
    hover_light: HoverLight,
    navigation_slot: UINavSlot
}

pub type MenuButton = (ButtonBundle, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);

impl ButtonBundle{
    pub fn menu_button(color: Color, text: &str, slot: UINavSlot) -> MenuButton {
        (
            Self{
                button: Button,
                container: Node {
                    width: Val::Px(350.0),
                    height: Val::Px(70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                background_color: BackgroundColor(color),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                border_color: BorderColor::from(Color::WHITE.with_alpha(0.3)),
                hover_light: HoverLight {
                    amount: 0.0,
                    max: 0.3,
                    speed: 2.0,
                    base: color,
                },
                navigation_slot: slot,
            },
            children![LabelBundle::button_label(text)]
        )
    }

    pub fn spawn_main_menu_buttons(commands: &mut RelatedSpawnerCommands<ChildOf>) {
        commands.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(Val::Px(50.0),Val::Px(50.0),Val::Px(50.0),Val::ZERO),
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
        ))
            .with_children(|parent|{

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.2, 0.6, 0.9),
                                                       "Offline Play", UINavSlot::row(0)))
                    .observe(on_offline);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.6, 0.3, 0.9),
                                                       "Online Play", UINavSlot::row(1)))
                    .observe(on_online);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.5, 0.5, 0.5),
                                                       "Settings", UINavSlot::row(2)))
                    .observe(on_settings);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.8, 0.2, 0.2),
                                                       "Exit", UINavSlot::row(3)))
                    .observe(on_exit);
            });
    }
}