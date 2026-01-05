use std::time::Duration;
use crate::components::ui::effects::{HoverLight, HoverLightColor};
use bevy::prelude::*;
use bevy::prelude::*;
use bevy_tween::combinator::{tween, tween_exact};
use bevy_tween::interpolate::{background_color, background_color_to};
use bevy_tween::prelude::*;
use bevy_tween::tween::apply_component_tween_system;
use crate::events::ui::widgets::ButtonPressed;
use crate::utils::{lighten_color, DEFAULT_LIGHTEN_AMOUNT};

pub fn detect_button_press(
    button_query: Query<Entity, (With<Button>, With<Interaction>)>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for entity in &button_query {
        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Pressed {
                commands.trigger(ButtonPressed(entity));
                return;
            }
        }
    }
}

pub fn handle_ui_hover_light(
    mut commands: Commands,
    query: Query<(Entity, &Interaction, &BackgroundColor, Option<&HoverLightColor>),
        (Changed<Interaction>, With<HoverLight>)>,
) {
    for (entity, interaction, background_color, maybe_custom_colors) in &query {
        let base = background_color.0;

        let hover_color = if let Some(custom) = maybe_custom_colors {
            custom.hover_color
        } else {

            lighten_color(base, DEFAULT_LIGHTEN_AMOUNT)
        };

        let (target_color, duration) = match *interaction {
            Interaction::Hovered => (hover_color, Duration::from_millis(150)),
            _ => (base, Duration::from_millis(200)),
        };

        let target = entity.into_target();
        let mut bg_color_state = target.state(background_color.0);

        commands.animation().insert(tween_exact(
            Duration::ZERO..duration,
            EaseKind::CubicInOut,
            bg_color_state.with(background_color_to(target_color)),
        ));
    }
}