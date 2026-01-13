use crate::bundles::{App, Plugin, Update};
use crate::systems::detect_button_press;
use crate::systems::widgets::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                u_ui_hover_light,
                u_slider_visuals,
                detect_button_press
                ))
            .add_observer(update_selector);
    }
}