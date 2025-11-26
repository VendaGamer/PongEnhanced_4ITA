use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::bundles::ButtonBundle;
use crate::resources::controls::MenuAction;
use crate::utils::text::lighten_color;

#[derive(EntityEvent)]
pub struct ButtonPressed(Entity);

pub fn detect_button_press(
    button_query: Query<Entity, (With<Button>, With<Interaction>)>,
    interaction_query: Query<&Interaction>,
    mut commands: Commands,
) {
    for entity in &button_query {
        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Pressed {
                commands.trigger(ButtonPressed(entity));
                println!("TEST");
                return;
            }
        }
    }
}

pub fn lighten_buttons_on_hover(
    button_query: Query<(Entity, &mut BackgroundColor), With<Button>>,
    interaction_query: Query<&Interaction>
){
    for (entity, color) in &button_query {

        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Hovered {

                color = BackgroundColor::from(lighten_color(color.0));

            }
        }
    }
}