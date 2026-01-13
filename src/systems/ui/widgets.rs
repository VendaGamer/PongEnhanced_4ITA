use crate::bundles::widgets::*;
use crate::components::ui::effects::{HoverLight, HoverLightColor};
use crate::components::ui::{Dropdown, OptionSelector, SelectorButton, SelectorText};
use crate::models::ui::option::UIOption;
use crate::utils::{lighten_color, DEFAULT_LIGHTEN_AMOUNT, MODERN_THEME};
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, TrackClick};
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::interpolate::background_color_to;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use std::time::Duration;
use crate::events::ui::widgets::{ButtonPressed, OptionChanged};

pub const BUTTON_PADDING: Val = Val::Px(20.0);
pub const PIXEL_BORDER: f32 = 3.0; // Classic pixel border width
pub const BUTTON_OUTLINE: Outline = Outline::new(Val::Px(PIXEL_BORDER), Val::ZERO, Color::BLACK);


pub fn u_slider_visuals(
    sliders: Query<(Entity, &SliderValue, &SliderRange), Changed<SliderValue>>,
    children: Query<&Children>,
    mut thumbs: Query<&mut Node, With<SliderThumb>>,
) {
    for (slider_entity, value, range) in sliders.iter() {
        for child in children.iter_descendants(slider_entity) {
            if let Ok(mut thumb_node) = thumbs.get_mut(child) {
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
            }
        }
    }
}

pub fn u_ui_hover_light(
    mut commands: Commands,
    query: Query<(Entity, Ref<Interaction>, &HoverLight, Option<&HoverLightColor>),
    (Changed<Interaction>, With<BackgroundColor>)>,
) {
    for (entity, interaction, hover_light, maybe_custom_colors) in &query {
        let base_color = hover_light.0;

        if interaction.is_added() || !interaction.is_changed() {
            continue;
        }

        let hover_color = if let Some(custom) = maybe_custom_colors {
            custom.hover_color
        } else {
            lighten_color(base_color, DEFAULT_LIGHTEN_AMOUNT)
        };


        let mut target = entity.into_target();

        match *interaction {
            Interaction::Hovered => {


                commands.entity(entity).animation().insert_tween_here(
                    Duration::from_millis(250),
                    EaseKind::CubicInOut,
                    target.state(base_color).with(background_color_to(hover_color))
                );
            },
            Interaction::None => {

                commands.entity(entity).animation().insert_tween_here(
                    Duration::from_millis(250),
                    EaseKind::CubicInOut,
                    target.state(hover_color).with(background_color_to(base_color))
                );
            }
            _ => {}
        };
    }
}

pub fn w_button(color: Color, size: Vec2, text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(BUTTON_PADDING),
            border: UiRect::all(Val::Px(PIXEL_BORDER)),
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        Outline::new(Val::Px(PIXEL_BORDER), Val::ZERO, MODERN_THEME.outline),
        HoverLight(color),
        Children::spawn_one(LabelBundle::button_label(text)),
    )
}

pub fn w_menu_title(text: &'static str) -> impl Bundle {
    (
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::bottom(Val::Px(PIXEL_BORDER * 2.0)),
            ..default()
        },
        BorderColor::from(MODERN_THEME.accent),
        Text::new(text),
        TextFont {
            font_size: 72.0,
            ..default()
        },
        TextColor(MODERN_THEME.text_bright),

    )
}

pub fn w_slider(min: f32, max: f32, current: f32, tab_index: i32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(50),
            width: percent(100),
            ..default()
        },
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderValue(current),
        SliderRange::new(min, max),
        TabIndex(tab_index),
        Children::spawn((
            Spawn((
                Node {
                    height: px(12),
                    border: UiRect::all(px(PIXEL_BORDER)),
                    ..default()
                },
                BackgroundColor(MODERN_THEME.slider_track),
                BorderColor::from(MODERN_THEME.border),
                BorderRadius::ZERO,
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(20),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                Children::spawn_one(w_slider_thumb(Vec2::new(20.0,20.0)))
            )),
        )),
    )
}

pub fn w_menu_section() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(30.0)),
            margin: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(PIXEL_BORDER * 1.5)),
            ..default()
        },
        BackgroundColor(MODERN_THEME.section_bg),
        BorderColor::from(MODERN_THEME.border_dark),
        BorderRadius::ZERO,
    )
}


pub fn w_slider_thumb(size: Vec2) -> impl Bundle {
    (
        SliderThumb,
        Node {
            display: Display::Flex,
            width: px(size.x),
            height: px(size.y),
            position_type: PositionType::Absolute,
            left: percent(0),
            border: UiRect::all(px(PIXEL_BORDER)),
            ..default()
        },
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        BackgroundColor(MODERN_THEME.slider_thumb),
    )
}

pub fn w_dropdown(options: Vec<UIOption>, selected: usize, tab_index: i32) -> impl Bundle {
    (
        Dropdown { options, selected },
        Node {
            width: Val::Px(300.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(PIXEL_BORDER)),
            ..default()
        },
        BackgroundColor(MODERN_THEME.panel_bg),
        BorderColor::from(MODERN_THEME.border),
        BorderRadius::ZERO,
        HoverLight(MODERN_THEME.panel_bg),
        TabIndex(tab_index),
    )
}

pub fn w_selector(options: Vec<UIOption>, selected: usize, tab_index: i32, label: &str) -> impl Bundle {
    (
        OptionSelector { options, selected },
        Node {
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(20.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        Children::spawn(
            Spawn((
                w_button(MODERN_THEME.button, Vec2::new(40.0, 40.0), "<"),
                SelectorButton
            )),
            Spawn((
                Node {
                    width: Val::Px(450.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(PIXEL_BORDER)),
                    ..default()
                },
                BackgroundColor(MODERN_THEME.panel_bg),
                BorderColor::from(MODERN_THEME.border),
                BorderRadius::ZERO,

            )),

        )
    )

    root.with_children(|parent| {
        let selector_entity = parent.target_entity();
        parent
            .append_button()
            .insert()
            .observe(

            );

        parent.spawn((

            TabIndex(tab_index),
            children![
                    LabelBundle::button_label(label),
                    (
                        LabelBundle::button_label(""),
                        SelectorText { selector_entity }
                    )
                ],
        ));


        parent
            .append_button(MODERN_THEME.button, Vec2::new(40.0, 40.0), ">")
            .insert(SelectorButton {
                selector: selector_entity,
            })
            .observe(
                |pressed: On<ButtonPressed>,
                 mut selectors: Query<&mut OptionSelector>,
                 buttons: Query<&SelectorButton>,
                 mut commands: Commands,
                | {
                    if let Ok(button) = buttons.get(pressed.event_target()) {
                        if let Ok(mut selector) = selectors.get_mut(button.selector) {
                            selector.cycle_next();
                            commands.trigger(OptionChanged(button.selector));
                        }
                    }
                },
            );
    });

    root
}

pub fn w_menu_button(color: Color, text: &str) -> impl Bundle {
    w_button(color, Vec2::new(350.0, 70.0), text)
}

pub fn update_selector(
    pressed: On<ButtonPressed>,
    mut selectors: Query<&mut OptionSelector>,
    button: Query<&ChildOf, With<SelectorButton>>,
    mut commands: Commands)
{

    if let Ok(child_of) = button.get(pressed.event_target()) {
        let selector_entity = child_of.parent();
        if let Ok(mut selector) = selectors.get_mut(selector_entity) {
        selector.cycle_prev();
        commands.trigger(OptionChanged(selector_entity));
        }
    }
}