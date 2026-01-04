use crate::components::ui::effects::HoverLight;
use crate::utils::text::lighten_color;
use bevy::prelude::*;
use crate::events::ui::widgets::ButtonPressed;

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
    time: Res<Time>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut HoverLight)>,
) {
    for (interaction, mut bg, mut hover) in &mut query {
        let dt = time.delta_secs();

        let target = match *interaction {
            Interaction::Hovered => hover.max,
            _ => 0.0,
        };

        hover.amount = move_towards(hover.amount, target, hover.speed * dt);
        bg.0 = lighten_color(hover.base, hover.amount);
    }
}


fn move_towards(current: f32, target: f32, max_delta: f32) -> f32 {
    if (target - current).abs() <= max_delta {
        target
    } else if current < target {
        current + max_delta
    } else {
        current - max_delta
    }
}