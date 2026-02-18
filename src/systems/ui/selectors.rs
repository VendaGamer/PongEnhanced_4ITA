use crate::components::ui::{Selector, SelectorText};
use bevy::prelude::*;

pub fn u_selector_text(
    selectors: Query<(Entity, &Selector), Changed<Selector>>,
    mut texts: Query<&mut Text, With<SelectorText>>,
    children: Query<&Children>,
) {
    for (selector_entity, selector) in &selectors {
        for child in children.iter_descendants(selector_entity) {
            if let Ok(mut text) = texts.get_mut(child) {
                text.0.clear();
                selector.push_current_string(&mut text.0);
                break;
            }
        }
    }
}