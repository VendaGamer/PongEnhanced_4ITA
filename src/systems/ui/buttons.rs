use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::resources::controls::MenuAction;

#[derive(EntityEvent)]
pub struct ButtonPressed(Entity);

pub fn detect_button_press(
    action_query: Query<&ActionState<MenuAction>>,
    button_query: Query<(Entity, &Interaction), (With<Button>, Changed<Interaction>)>,
    mut commands: Commands,
) {
    // Keyboard/gamepad confirm
    for action_state in &action_query {
        if action_state.just_pressed(&MenuAction::Confirm) {
            for (entity, interaction) in &button_query {
                if *interaction == Interaction::Hovered {
                    commands.trigger(ButtonPressed(entity));
                }
            }
        }
    }
}