use crate::bundles::widgets::*;
use crate::components::ui::effects::{HoverLight, HoverLightColor};
use crate::components::ui::{Dropdown, Selector, SelectorButton, SelectorText, SourceHandle, UIOptionProvider};
use crate::events::widgets::{ButtonPressed, OptionChanged};
use crate::resources::MenuAction;
use crate::utils::{lighten_color, DEFAULT_LIGHTEN_AMOUNT, MODERN_THEME};
use bevy::ecs::relationship::{RelatedSpawnerCommands, Relationship};
use bevy::input_focus::directional_navigation::DirectionalNavigation;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::input_focus::{AutoFocus, InputFocus, InputFocusVisible};
use bevy::math::{CompassOctant, CompassQuadrant};
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy::ui::Checked;
use bevy::ui_widgets::{
    Checkbox, Slider, SliderPrecision, SliderRange, SliderThumb, SliderValue, TrackClick,
};
use bevy_tween::interpolate::background_color_to;
use bevy_tween::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use std::sync::Arc;
use std::time::Duration;
use bevy_simple_text_input::{TextInput, TextInputSubmitMessage, TextInputTextColor, TextInputTextFont};
use crate::events::gameplay::UINavigated;

pub const BUTTON_PADDING: Val = Val::Px(20.0);
pub const PIXEL_BORDER: UiRect = UiRect::all(Val::Px(3.0));
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
    query: Query<
        (
            Entity,
            Ref<Interaction>,
            &HoverLight,
            Option<&HoverLightColor>,
        ),
        (Changed<Interaction>, With<BackgroundColor>),
    >,
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

        let target = entity.into_target();

        match *interaction {
            Interaction::Hovered => {
                commands.entity(entity).animation().insert_tween_here(
                    Duration::from_millis(250),
                    EaseKind::CubicInOut,
                    target
                        .state(base_color)
                        .with(background_color_to(hover_color)),
                );
            }
            Interaction::None => {
                commands.entity(entity).animation().insert_tween_here(
                    Duration::from_millis(250),
                    EaseKind::CubicInOut,
                    target
                        .state(hover_color)
                        .with(background_color_to(base_color)),
                );
            }
            _ => {}
        };
    }
}

pub fn w_button(color: Color, text: &str, size: Val2) -> impl Bundle {
    (
        Button,
        Node {
            width: size.x,
            height: size.y,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(BUTTON_PADDING),
            border: PIXEL_BORDER,
            ..default()
        },
        AutoFocus,
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
        AutoFocus,
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        BackgroundColor(MODERN_THEME.slider_thumb),
    )
}

pub fn w_dropdown(
    options: Arc<dyn UIOptionProvider>,
    selected: usize,
    tab_index: i32,
) -> impl Bundle {
    (
        Dropdown { selected, options },
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

#[derive(Component)]
pub struct SelectorBar;

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
    (Checkbox, Checked::default())
}

pub fn w_menu_button(color: Color, text: &str) -> impl Bundle {
    w_button(color, text, Val2::new(Val::Px(350.0), Val::Px(70.0)))
}

pub fn update_selector(
    pressed: On<ButtonPressed>,
    mut selectors: Query<&mut Selector>,
    button: Query<(&ChildOf, &SelectorButton)>,
    mut commands: Commands,
) {
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
                selected_index: selector.selected,
            });
        }
    }
}

pub fn w_row_container(gap: Val) -> impl Bundle {
    Node {
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        column_gap: gap,
        display: Display::Flex,
        ..default()
    }
}

pub fn w_col_container(gap: Val) -> impl Bundle {
    Node {
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::Wrap,
        row_gap: gap,
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
                Node {
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
const INITIAL_REPEAT_DELAY: f32 = 0.5;
const REPEAT_DELAY: f32 = 0.15;
const DEADZONE: f32 = 0.3;

pub fn t_navigate_element(
    mut input_focus: ResMut<InputFocusVisible>,
    mut navigation: DirectionalNavigation,
    mut last_direction: Local<Option<CompassQuadrant>>,
    mut auto_nav_delay: Local<f32>,
    state: Single<&ActionState<MenuAction>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if let Some(data) = state.dual_axis_data(&MenuAction::Navigate) {
        let current = get_quadrant(data.pair);

        *auto_nav_delay -= time.delta_secs();

        if *auto_nav_delay < -0.001 {
            *auto_nav_delay = 0.15;
            try_navigate(current, &mut navigation, &mut commands);
            return;
        }

        if current.eq(&*last_direction) {
            return;
        }

        if !input_focus.0 {
            input_focus.0 = true;
            *last_direction = current;
            return;
        }

        *auto_nav_delay = 0.7;
        try_navigate(current, &mut navigation, &mut commands);
        *last_direction = current;
    }
}

fn get_quadrant(input: Vec2) -> Option<CompassQuadrant> {
    let magnitude = input.length();

    if magnitude < DEADZONE {
        return None;
    }

    let normalized = input / magnitude;

    let abs_x = normalized.x.abs();
    let abs_y = normalized.y.abs();

    if abs_x > abs_y {
        if normalized.x > 0.0 {
            Some(CompassQuadrant::East)
        } else {
            Some(CompassQuadrant::West)
        }
    } else {
        if normalized.y > 0.0 {
            Some(CompassQuadrant::North)
        } else {
            Some(CompassQuadrant::South)
        }
    }
}

#[inline]
fn try_navigate(
    dir: Option<CompassQuadrant>,
    navigation: &mut DirectionalNavigation,
    commands: &mut Commands,
) {
    if let Some(dir) = dir {
        let octant = quadrant_to_octant(&dir);

        if matches!(&dir, CompassQuadrant::West | CompassQuadrant::East) {
            commands.queue(move |world: &mut World| {
                let focus = world.resource::<InputFocus>();

                if let Some(focused) = focus.0 {
                    let parent_entity = world
                        .entity(focused)
                        .get::<ChildOf>()
                        .map(|child_of| child_of.0);

                    if let Some(parent_id) = parent_entity {
                        if let Some(mut selector) =
                            world.entity_mut(parent_id).get_mut::<Selector>()
                        {
                            if dir.eq(&CompassQuadrant::West) {
                                selector.prev();
                            } else {
                                selector.next();
                            }

                            world.trigger(OptionChanged {
                                entity: parent_id,
                                selected_index: 0,
                            });
                        } else if let Some(parent) = world.entity(parent_id).get::<ChildOf>() {
                            let mut par = world.entity_mut(parent.0);

                            if let Some(value) = par.get::<SliderValue>() {
                                if dir.eq(&CompassQuadrant::West) {
                                    par.insert(SliderValue(value.0 - 1.0));
                                } else {
                                    par.insert(SliderValue(value.0 + 1.0));
                                }
                            }
                        }
                    }
                }
            });
        }

        match navigation.navigate(octant) {
            Ok(entity) => {
                println!("Navigated {octant:?} successfully. {entity} is now focused.");
                commands.trigger(UINavigated {
                    direction: octant,
                    entity
                })
            }
            Err(e) => {
                println!("Navigation failed: {e}");
            }
        }
    }
}

pub fn quadrant_to_octant(quadrant: &CompassQuadrant) -> CompassOctant {
    match quadrant {
        CompassQuadrant::North => CompassOctant::North,
        CompassQuadrant::East => CompassOctant::East,
        CompassQuadrant::South => CompassOctant::South,
        CompassQuadrant::West => CompassOctant::West,
    }
}

pub fn u_button_press(
    focused: Res<InputFocus>,
    visible: Res<InputFocusVisible>,
    state: Single<&ActionState<MenuAction>>,
    mut commands: Commands,
) {
    if !visible.0 {
        return;
    }

    if state.just_pressed(&MenuAction::Confirm) {
        if let Some(entity) = focused.0 {
            commands.entity(entity).insert(Interaction::Pressed);
        }
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

#[derive(Deref)]
pub struct SliderEntities<'a> {
    #[deref]
    pub root: EntityCommands<'a>,
    pub track: Entity,
    pub thumb: Entity,
}

#[derive(Deref)]
pub struct SelectorEntities<'a> {
    #[deref]
    pub root: EntityCommands<'a>,
    pub left_button: Entity,
    pub right_button: Entity,
    pub bar: Entity,
}

#[derive(Deref)]
pub struct InputEntities<'a> {
    pub root: Entity,
    pub title: Entity,
    #[deref]
    pub input: EntityCommands<'a>,
}

pub trait WidgetsExtCommands {
    fn spawn_slider_custom(
        &mut self,
        min: f32,
        max: f32,
        cur: f32,
        size: Val2,
    ) -> SliderEntities<'_>;

    #[inline]
    fn spawn_slider_interactable(
        &mut self,
        min: f32,
        max: f32,
        cur: f32,
        entities: &mut Vec<Entity>) -> SliderEntities<'_>
    {
        let slider = self.spawn_slider(min, max, cur);

        entities.push(slider.thumb);

        slider
    }

    #[inline]
    fn spawn_slider(&mut self, min: f32, max: f32, cur: f32) -> SliderEntities<'_> {
        const SIZE: Val2 = Val2::new(Val::Percent(100.0), Val::Px(50.0));

        self.spawn_slider_custom(min, max, cur, SIZE)
    }

    #[inline]
    fn spawn_selector_interactable(
        &mut self,
        options_provider: SourceHandle<dyn UIOptionProvider>,
        selected: usize,
        label: impl Into<String>,
        entities: &mut Vec<Entity>) -> SelectorEntities<'_> {

        let selector = self.spawn_selector(options_provider, selected, label);
        
        entities.push(selector.bar);
        
        selector
    }

    fn spawn_selector_custom(
        &mut self,
        options_provider: SourceHandle<dyn UIOptionProvider>,
        selected: usize,
        label: impl Into<String>,
        size: Val2,
    ) -> SelectorEntities<'_>;

    #[inline]
    fn spawn_selector(
        &mut self,
        options_provider: SourceHandle<dyn UIOptionProvider>,
        selected: usize,
        label: impl Into<String>,
    ) -> SelectorEntities<'_> {
        const SIZE: Val2 = Val2::new(Val::Px(460.0), Val::Px(30.0));

        self.spawn_selector_custom(options_provider, selected, label, SIZE)
    }

    #[inline]
    fn spawn_input(
        &mut self,
        label: impl Into<String>
    ) -> InputEntities<'_> {
        const SIZE: Val2 = Val2::new(Val::Px(460.0), Val::Px(30.0));

        self.spawn_input_custom(label, SIZE)
    }

    fn spawn_input_custom(
        &mut self,
        label: impl Into<String>,
        size: Val2,
    ) -> InputEntities<'_>;
}

impl<'w, R: Relationship> WidgetsExtCommands for RelatedSpawnerCommands<'w, R> {
    fn spawn_slider_custom(
        &mut self,
        min: f32,
        max: f32,
        cur: f32,
        size: Val2,
    ) -> SliderEntities<'_> {
        let mut root = self.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                justify_items: JustifyItems::Center,
                column_gap: px(4),
                height: size.y,
                width: size.x,
                ..default()
            },
            Hovered::default(),
            Slider {
                track_click: TrackClick::Snap,
            },
            SliderPrecision(0),
            SliderValue(cur),
            SliderRange::new(min, max),
        ));

        let mut commands = root.commands();
        let thumb: Entity;
        let track: Entity;

        {
            track = commands
                .spawn((
                    Node {
                        height: px(12),
                        border: PIXEL_BORDER,
                        ..default()
                    },
                    BackgroundColor(MODERN_THEME.slider_track),
                    BorderColor::from(MODERN_THEME.border),
                    BorderRadius::ZERO,
                ))
                .id();

            let thumb_root = commands
                .spawn((Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(20),
                    top: px(15),
                    bottom: px(0),
                    ..default()
                },))
                .id();

            thumb = commands.spawn(w_slider_thumb(Vec2::new(20.0, 20.0))).id();
            commands.entity(thumb_root).add_child(thumb);
            root.add_children(&[track, thumb_root]);
        }

        SliderEntities { root, track, thumb }
    }

    fn spawn_selector_custom(
        &mut self,
        options_provider: SourceHandle<dyn UIOptionProvider>,
        selected: usize,
        label: impl Into<String>,
        size: Val2,
    ) -> SelectorEntities<'_> {
        let mut root = self.spawn((
            Selector {
                options_provider,
                selected,
            },
            Node {
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Row,
                row_gap: Val::Px(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
        ));

        let mut commands = root.commands();

        let l_but: Entity;
        let r_but: Entity;
        let bar: Entity;

        {
            l_but = commands
                .spawn(w_button(
                    MODERN_THEME.button,
                    "<",
                    Val2::new(size.y, size.y),
                ))
                .insert(SelectorButton(false))
                .id();

            bar = commands.spawn((
                    Node {
                        width: size.x,
                        height: size.y,
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        justify_items: JustifyItems::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(15.0)),
                        border: PIXEL_BORDER,
                        ..default()
                    },
                    SelectorBar,
                    AutoFocus,
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
                    )),
                ))
                .id();

            r_but = commands
                .spawn(w_button(
                    MODERN_THEME.button,
                    ">",
                    Val2::new(size.y, size.y),
                ))
                .insert(SelectorButton(true))
                .id();

            root.add_children(&[l_but, bar, r_but]);
        }

        SelectorEntities {
            root,
            bar,
            left_button: l_but,
            right_button: r_but,
        }
    }

    fn spawn_input_custom(
        &mut self,
        label: impl Into<String>,
        size: Val2) -> InputEntities<'_> {

        let target = self.target_entity();
        let commands = self.commands_mut();

        let root = commands.spawn((
            R::from(target),
            Node {
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        })).id();


        let title: Entity;
        let input: Entity;

        {
            title = commands.spawn(LabelBundle::button_label(label)).id();

            input = commands.spawn((
                     Node {
                         width: size.x,
                         height: size.y,
                         margin: UiRect::all(Val::Px(10.0)),
                         justify_content: JustifyContent::SpaceBetween,
                         justify_items: JustifyItems::Center,
                         align_items: AlignItems::Center,
                         padding: UiRect::all(Val::Px(15.0)),
                         border: PIXEL_BORDER,
                         ..default()
                     },
                     AutoFocus,
                     BorderColor::all(MODERN_THEME.border),
                     BackgroundColor(MODERN_THEME.panel_bg),
                     BorderRadius::ZERO,
                     TextInput,
                     TextInputTextFont(TextFont {
                         font_size: 34.0,
                         ..default()
                     }),
                     TextInputTextColor(TextColor(MODERN_THEME.text_normal)),
                )).id();

            commands.entity(root).add_children(&[title, input]);
        }

        InputEntities{
            root,
            title,
            input: commands.entity(input),
        }
    }
}
