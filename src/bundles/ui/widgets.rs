use std::convert::Into;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy::ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, TrackClick};
use crate::bundles::{children, default, BackgroundColor, BorderRadius, Bundle, ChildOf, Color, Node, Spawn, Text, TextColor, TextFont, UiRect, Val};
use crate::bundles::ui::main_menu::*;
use crate::components::ui::Dropdown;
use crate::components::ui::effects::HoverLight;
use crate::components::ui::navigation::{OptionSelector, UINavSlot};
use crate::models::ui::option::UIOption;
use crate::systems::ButtonPressed;

#[derive(Bundle)]
pub struct SliderThumbBundle {
    pub thumb: SliderThumb,
    pub container: Node,
    pub background: BackgroundColor,
    pub border_radius: BorderRadius,
}

impl Default for SliderThumbBundle {
    fn default() -> Self {
        Self{
            thumb: SliderThumb,
            container: Node {
                width: Val::Px(20.0),
                height: Val::Px(32.0),
                ..default()
            },
            background: BackgroundColor(Color::srgb(0.8, 0.8, 0.85)),
            border_radius: BorderRadius::all(Val::Px(3.0)),
        }
    }
}


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
    pub fn new(min: f32, max: f32, current: f32, slot: UINavSlot) -> (SliderBundle, SpawnRelatedBundle<ChildOf, Spawn<SliderThumbBundle>>) {
        (
            Self {
                slider: Slider {
                    track_click: TrackClick::Drag
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
            },
            children![SliderThumbBundle::default()]
        )

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
pub type StatusLabel = (Node, BackgroundColor, BorderRadius, SpawnRelatedBundle<ChildOf, (Spawn<Text>, Spawn<TextFont>, Spawn<TextColor>)>);

impl LabelBundle {
    pub fn button_label(text: &str) -> Self{
        Self{
            text: Text::new(text),
            font: TextFont {
                font_size: 32.0,
                font_smoothing: FontSmoothing::None,
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
                        font_smoothing: FontSmoothing::None,
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
    outline: Outline,
    hover_light: HoverLight,
    navigation_slot: UINavSlot
}

pub type ButtonT = (ButtonBundle, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);
const BUTTON_PADDING: Val = Val::Px(30.0);
const BUTTON_OUTLINE: Outline = Outline::new(Val::Px(5.0),Val::ZERO, Color::WHITE);

const BUTTON_BORDER: Color = Color::linear_rgb(0.8, 0.8, 0.8);

impl ButtonBundle{
    pub fn menu_button(color: Color, text: &str, slot: UINavSlot) -> ButtonT {
        (
            Self{
                button: Button,
                container: Node {
                    width: Val::Px(350.0),
                    height: Val::Px(70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(BUTTON_PADDING),
                    ..default()
                },
                background_color: BackgroundColor(color),
                border_radius: BorderRadius::ZERO,
                border_color: BorderColor::from(BUTTON_BORDER),
                outline: BUTTON_OUTLINE,
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

    pub fn navigate_button(text: &str) -> ButtonT {
        (
           
        )
    }

    pub fn spawn_main_menu_buttons(commands: &mut RelatedSpawnerCommands<ChildOf>) {
        commands.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(BUTTON_PADDING,BUTTON_PADDING,BUTTON_PADDING,Val::ZERO),
                ..default()
            },
            Outline::new(Val::Px(5.0), Val::ZERO, Color::linear_rgb(0.5,0.5,0.5)),
            BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
        ))
            .with_children(|parent|{

                parent.spawn(ButtonBundle::menu_button(
                    Color::srgb(0.2, 0.6, 0.9),
               "Offline Play",
                    UINavSlot::row(0))
                ).observe(on_offline);

                parent.spawn(ButtonBundle::menu_button(
                    Color::srgb(0.6, 0.3, 0.9),
               "Online Play",
                    UINavSlot::row(1))
                ).observe(on_online);

                parent.spawn(ButtonBundle::menu_button(
                    Color::srgb(0.5, 0.5, 0.5),
               "Settings", UINavSlot::row(2))
                ).observe(on_settings);

                parent.spawn(ButtonBundle::menu_button(
                    Color::srgb(0.8, 0.2, 0.2),
               "Exit",
                    UINavSlot::row(3))
                ).observe(on_exit);

            });
    }
}


#[derive(Bundle)]
pub struct OptionSelectorBundle
{
    pub selector: OptionSelector,
    pub container: Node,
    pub background_color: BackgroundColor,
    pub border_radius: BorderRadius,
    pub navigation_slot: UINavSlot,
}

pub type OptionSelectorT = (OptionSelectorBundle, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);



impl OptionSelectorBundle{

    pub fn spawn_new(commands: &mut RelatedSpawnerCommands<ChildOf>, options: Vec<UIOption>, selected: usize, slot: UINavSlot, label: &str) {

        commands.spawn((
            Node{
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                height: Val::Auto,
                width: Val::Auto,
                display: Display::Flex,
                ..default()
            },
        ))
        .with_children(|parent|{

                parent.spawn(ButtonBundle::navigate_button("<"))
                            .observe(|on_click: On<ButtonPressed>|{

                            });

                let val = parent.spawn(OptionSelectorBundle::new_select(options, selected, slot, label)).id();

                parent.spawn(ButtonBundle::navigate_button("<"))
                             .observe(|on_click: On<ButtonPressed>|{

                             });
        });

    }

    fn new_select(options: Vec<UIOption>, selected: usize, slot: UINavSlot, label: &str) -> OptionSelectorT {
        (
            Self {
                selector: OptionSelector { options, selected },
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
                navigation_slot: slot,
            },
            children![LabelBundle::button_label(label)]
        )
    }
}