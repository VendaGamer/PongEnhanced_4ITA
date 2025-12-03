use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, TrackClick};
use crate::bundles::widgets::*;
use crate::components::{AreaShape, GameMode};
use crate::components::ui::effects::HoverLight;
use crate::components::ui::{Dropdown, Menu, SelectorButton};
use crate::components::ui::navigation::{OptionSelector, SelectorText, UINavSlot};
use crate::models::game::fullscreen::ScreenMode;
use crate::models::ui::option::UIOption;
use crate::systems::ButtonPressed;

pub const BUTTON_PADDING: Val = Val::Px(30.0);
pub const BUTTON_OUTLINE: Outline = Outline::new(Val::Px(5.0),Val::ZERO, Color::WHITE);
pub const BUTTON_BORDER: Color = Color::linear_rgb(0.8, 0.8, 0.8);


pub trait WidgetSpawnExt {
    fn append_selector(&mut self, options: Vec<UIOption>, selected: usize, slot: UINavSlot, label: &str) -> EntityCommands<'_>;
    fn append_dropdown(&mut self, options: Vec<UIOption>, selected: usize, slot: UINavSlot) -> EntityCommands<'_>;
    fn append_menu_section(&mut self) -> EntityCommands<'_>;
    fn append_slider(&mut self, min: f32, max: f32, current: f32, slot: UINavSlot) -> EntityCommands<'_>;
    fn append_button(&mut self, color: Color, size: Vec2, text: &str) -> EntityCommands<'_>;
    fn append_menu_button(&mut self, color: Color, text: &str, slot: UINavSlot) -> EntityCommands<'_>;

    fn append_menu_title(&mut self, text: &'static str) -> EntityCommands<'_>;
}


impl<'w> WidgetSpawnExt for RelatedSpawnerCommands<'w, ChildOf> {
    fn append_selector(&mut self, options: Vec<UIOption>, selected: usize, slot: UINavSlot, label: &str) -> EntityCommands<'_> {

        let mut root = self.spawn(Node{
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(20.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        });

        root.with_children(|parent| {

            let left_button = parent
                .append_button(Color::srgb(0.2, 0.2, 0.25), Vec2::new(25.0, 25.0), "<")
                .observe(
                    |   pressed:On<ButtonPressed>,
                        mut selectors: Query<&mut OptionSelector>,
                        buttons: Query<&SelectorButton>
                    | {
                        if let Ok(button) = buttons.get(pressed.event_target()) {
                            if let Ok(mut selector) = selectors.get_mut(button.selector) {
                                selector.cycle_prev();
                            }
                        }
                    }).id();

            let selector_entity = parent.spawn((
                OptionSelector { options, selected },
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                BorderRadius::all(Val::Px(5.0)),
                slot,
                children![LabelBundle::button_label(label)]
            )).id();

            parent.commands().entity(selector_entity).with_child((
                Text::new("Empty"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
                SelectorText { selector_entity },
            ));


            parent.commands().entity(left_button).insert(SelectorButton{
                selector: selector_entity,
            });


            parent.append_button(Color::srgb(0.2, 0.2, 0.25), Vec2::new(25.0, 25.0), ">")
                .insert(SelectorButton{
                    selector: selector_entity,
                })
                .observe(
                |   pressed:On<ButtonPressed>,
                    mut selectors: Query<&mut OptionSelector>,
                    buttons: Query<&SelectorButton>
                | {
                    if let Ok(button) = buttons.get(pressed.event_target()) {
                        if let Ok(mut selector) = selectors.get_mut(button.selector) {
                            selector.cycle_next();
                        }
                    }
                });

        });

        root
    }
    fn append_dropdown(&mut self, options: Vec<UIOption>, selected: usize, slot: UINavSlot) -> EntityCommands<'_> {

        self.spawn((
            Dropdown { options, selected },
            Node {
               width: Val::Px(300.0),
               height: Val::Px(50.0),
               margin: UiRect::all(Val::Px(10.0)),
               justify_content: JustifyContent::Center,
               align_items: AlignItems::Center,
               ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
            BorderRadius::all(Val::Px(5.0)),
            HoverLight {
                amount: 0.0,
                max: 0.2,
                speed: 3.0,
                base: Color::srgb(0.2, 0.2, 0.25),
            },
            slot
        ))

    }

    fn append_menu_section(&mut self) -> EntityCommands<'_> {

        self.spawn((
               Node {
                   flex_direction: FlexDirection::Column,
                   padding: UiRect::all(Val::Px(30.0)),
                   margin: UiRect::all(Val::Px(10.0)),
                   ..default()
               },
               BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
               BorderRadius::all(Val::Px(10.0)),
            ))

    }

    fn append_slider(&mut self, min: f32, max: f32, current: f32, slot: UINavSlot) -> EntityCommands<'_> {

        self.spawn((
            Slider {
                track_click: TrackClick::Drag
            },
            SliderRange::new(min, max),
            SliderValue(current),
            Node {
               width: Val::Px(300.0),
               height: Val::Px(40.0),
               margin: UiRect::all(Val::Px(10.0)),
               padding: UiRect::all(Val::Px(4.0)),
               ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
            BorderRadius::all(Val::Px(5.0)),
            HoverLight {
                amount: 0.0,
                max: 0.2,
                speed: 3.0,
                base: Color::srgb(0.2, 0.2, 0.25),
            },
            slot,
            children![(
                    SliderThumb,
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.8, 0.8, 0.85)),
                    BorderRadius::all(Val::Px(3.0)),
            )]))

    }

    fn append_button(&mut self, color: Color, size: Vec2, text: &str) -> EntityCommands<'_> {

    self.spawn(
        (
            Button,
            Node {
                width: Val::Px(size.x),
                height: Val::Px(size.y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(BUTTON_PADDING),
                ..default()
            },
            BackgroundColor(color),
            BorderRadius::ZERO,
            BorderColor::from(BUTTON_BORDER),
            BUTTON_OUTLINE,
            HoverLight {
                amount: 0.0,
                max: 0.3,
                speed: 2.0,
                base: color,
            },
            children![LabelBundle::button_label(text)]
        ))
    }

    fn append_menu_button(&mut self, color: Color, text: &str, slot: UINavSlot) -> EntityCommands<'_> {
        let mut button = self.append_button(color, Vec2::new(350.0, 70.0), text);

        button.insert(slot);

        button
    }

    fn append_menu_title(&mut self, text: &'static str) -> EntityCommands<'_> {

        self.spawn((
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
                Text::new(text),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ))

    }
}