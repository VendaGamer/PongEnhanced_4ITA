use crate::bundles::widgets::*;
use crate::components::ui::effects::{HoverLight, HoverLightColor};
use crate::components::ui::{Dropdown, Menu, OptionSelector, SelectorButton, SelectorText, SourceHandle, UIOptionProvider};
use crate::events::widgets::{ButtonPressed, OptionChanged};
use crate::resources::MenuAction;
use crate::utils::{lighten_color, DEFAULT_LIGHTEN_AMOUNT, MODERN_THEME};
use bevy::input_focus::directional_navigation::DirectionalNavigation;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::math::CompassOctant;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy::ui_widgets::{Checkbox, Slider, SliderPrecision, SliderRange, SliderThumb, SliderValue, ToggleChecked, TrackClick};
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::interpolate::background_color_to;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use leafwing_input_manager::action_state::ActionState;
use std::sync::Arc;
use std::time::Duration;
use bevy::ui::Checked;

pub const BUTTON_PADDING: Val = Val::Px(20.0);
pub const PIXEL_BORDER: UiRect = UiRect::all(Val::Px(3.0)); // Classic pixel border width
pub const BUTTON_OUTLINE: Outline = Outline::new(PIXEL_BORDER.bottom, Val::ZERO, Color::BLACK);


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
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        Outline::new(PIXEL_BORDER.bottom, Val::ZERO, MODERN_THEME.outline),
        HoverLight(color),
        Children::spawn_one(LabelBundle::button_label(text)),
    )
}

pub fn w_menu_title(text: impl Into<String>) -> impl Bundle {
    w_title(text, 72.0)
}

pub fn w_title(text: impl Into<String>, size: f32) -> impl Bundle {
    (
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            padding: UiRect::all(Val::Px(10.0)),
            border: PIXEL_BORDER,
            ..default()
        },
        BorderColor::from(MODERN_THEME.accent),
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(MODERN_THEME.text_bright),
    )
}

pub fn w_slider(min: f32, max: f32, current: f32) -> impl Bundle {
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
        SliderPrecision(0),
        SliderValue(current),
        SliderRange::new(min, max),
        Children::spawn((
            Spawn((
                Node {
                    height: px(12),
                    border: PIXEL_BORDER,
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
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(30.0)),
            margin: UiRect::all(Val::Px(10.0)),
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(MODERN_THEME.section_bg),
        BorderColor::all(MODERN_THEME.border_dark),
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
            border: PIXEL_BORDER,
            ..default()
        },
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        BackgroundColor(MODERN_THEME.slider_thumb),
    )
}

pub fn w_dropdown(options: Arc<dyn UIOptionProvider>, selected: usize, tab_index: i32) -> impl Bundle {
    (
        Dropdown {
            selected,
            options,
        },
        Node {
            width: Val::Px(300.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(MODERN_THEME.panel_bg),
        BorderColor::from(MODERN_THEME.border),
        BorderRadius::ZERO,
        HoverLight(MODERN_THEME.panel_bg),
        TabIndex(tab_index),
    )
}
pub fn w_selector(options_provider: SourceHandle<dyn UIOptionProvider>, selected: usize, label: impl Into<String>) -> impl Bundle {
    (
        OptionSelector {
            options_provider,
            selected
        },
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
        Children::spawn((
            Spawn((
                w_button(MODERN_THEME.button, Vec2::new(40.0, 40.0), "<"),
                SelectorButton(false),
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
                    border: PIXEL_BORDER,
                    ..default()
                },
                BackgroundColor(MODERN_THEME.panel_bg),
                BorderColor::from(MODERN_THEME.border),
                BorderRadius::ZERO,
                Children::spawn_one((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    Children::spawn((
                        Spawn(LabelBundle::button_label(label)),
                        Spawn((
                            TextFont {
                                font_size: 32.0,
                                font_smoothing: FontSmoothing::None,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            SelectorText,
                        )),
                    )),
                ))
            )),
            Spawn((
                w_button(MODERN_THEME.button, Vec2::new(40.0, 40.0), ">"),
                SelectorButton(true),
            ))
        ))
    )
}

pub fn w_section_header(text: &'static str) -> impl Bundle {
    (
        Node {
            margin: UiRect::new(Val::ZERO, Val::ZERO, Val::Px(20.0), Val::Px(10.0)),
            ..default()
        },
        Text::new(text),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(MODERN_THEME.accent),
    )
}

pub fn w_checkbox(state: bool) -> impl Bundle {
    (
        Checkbox,
        Checked::default(),
    )
}


pub fn w_menu_button(color: Color, text: &str) -> impl Bundle {
    w_button(color, Vec2::new(350.0, 70.0), text)
}

pub fn update_selector(
    pressed: On<ButtonPressed>,
    mut selectors: Query<&mut OptionSelector>,
    button: Query<(&ChildOf, &SelectorButton)>,
    mut commands: Commands)
{
    if let Ok((child_of, button)) = button.get(pressed.event_target()) {
        let selector_entity = child_of.parent();
        if let Ok(mut selector) = selectors.get_mut(selector_entity) {

            if button.0 {
                selector.next();
            } else {
                selector.prev();
            }

            commands.trigger(OptionChanged {
                entity: selector_entity,
                selected_index: selector.selected
            });
        }
    }
}

pub fn w_row_container(gap: f32) -> impl Bundle {
    Node{
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        column_gap: Val::Px(gap),
        display: Display::Flex,
        ..default()
    }
}

pub fn w_col_container(gap: f32) -> impl Bundle {
    Node{
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::Wrap,
        row_gap: Val::Px(gap),
        display: Display::Flex,
        ..default()
    }
}

pub fn w_area_container(size: f32, text: &'static str, visuals: impl Bundle) -> impl Bundle {
    (
        Node {
            align_items: AlignItems::Center,
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node{
                    width: Val::Px(size),
                    height: Val::Px(size),
                    border: PIXEL_BORDER,
                    ..default()
                },
                BackgroundColor(MODERN_THEME.section_bg),
                BorderColor::from(MODERN_THEME.border),
                Children::spawn_one(visuals)
            ),
            LabelBundle::button_label(text),
        ],
    )
}

pub fn t_button_press(
    button_query: Query<Entity, (With<Button>, With<Interaction>)>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for entity in &button_query {
        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Pressed {
                commands.trigger(ButtonPressed(entity));
            }
        }
    }
}

const FOCUSED_BORDER: Srgba = bevy::color::palettes::tailwind::AMBER_500;

pub fn u_navigate_element(
    query: Query<&ActionState<MenuAction>>,
    menu: Query<&Menu>,
    mut directional_navigation: DirectionalNavigation
) {
    if !menu.single().is_ok(){
        return;
    }
    
    let state = query.single().expect("Expected menu action state");

    if let Some(data) = state.dual_axis_data(&MenuAction::Navigate){

        if let Some(octant) = to_octant(data.pair){

            match directional_navigation.navigate(octant){
                Ok(entity) =>{
                    println!("Navigated {octant:?} successfully. {entity} is now focused.");
                },
                Err(e) =>{
                    println!("Navigation failed: {e}");
                }
            }
        }
    }
}

pub fn to_octant(vec: Vec2) -> Option<CompassOctant> {
    const THRESHOLD: f32 = 0.5;

    match (vec.x.abs() > THRESHOLD, vec.y.abs() > THRESHOLD) {
        (false, true) if vec.y > 0.0 => Some(CompassOctant::North),
        (false, true) if vec.y < 0.0 => Some(CompassOctant::South),
        (true, false) if vec.x < 0.0 => Some(CompassOctant::West),
        (true, false) if vec.x > 0.0 => Some(CompassOctant::East),
        _ => None
    }
}


pub fn u_highlight_focused_element(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut BorderColor)>,
) {
    for (entity, mut border_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            *border_color = BorderColor::all(FOCUSED_BORDER);
        } else {
            *border_color = BorderColor::all(MODERN_THEME.border);
        }
    }
}
