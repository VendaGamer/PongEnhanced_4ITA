use crate::events::ui::widgets::ButtonPressed;
use bevy::prelude::*;

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