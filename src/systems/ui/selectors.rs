use bevy::prelude::*;
use crate::components::ui::{OptionSelector, SelectorText};

pub fn update_selector_text(
    selectors: Query<&OptionSelector, Changed<OptionSelector>>,
    mut texts: Query<(&SelectorText, &mut Text)>,
) {
    for (selector_text, mut text) in &mut texts {
        if let Ok(selector) = selectors.get(selector_text.selector_entity) {
            text.0 = selector.get_current().into();
        }
    }
}

pub fn handle_selector_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selectors: Query<(&mut OptionSelector, &Interaction), Changed<Interaction>>,
) {
    for (mut selector, interaction) in &mut selectors {
        if *interaction == Interaction::Hovered {
            if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
                selector.cycle_next();
            }
            if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
                selector.cycle_prev();
            }
        }
    }
}