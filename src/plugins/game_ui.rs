use crate::bundles::{App, Commands, MessageReader, On, Plugin, ResMut, UiScale, Update};
use crate::events::widgets::SliderValueChanged;
use crate::systems::widgets::*;
use bevy::prelude::Query;
use bevy::ui_widgets::{SliderValue, ValueChange};
use bevy::window::WindowResized;
use crate::utils::FIXED_DIMENSIONS;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                u_ui_hover_light,
                u_slider_visuals,
                t_button_press,
                handle_ui_scaling,
                u_highlight_focused_element,
                u_navigate_element,
                ))
            .add_observer(t_slider_change)
            .add_observer(update_selector);
    }
}

fn handle_ui_scaling(
    mut ui_scale: ResMut<UiScale>,
    mut resized: MessageReader<WindowResized>)
{
    for event in resized.read() {

        let scale_x = event.width / FIXED_DIMENSIONS.x;
        let scale_y = event.height / FIXED_DIMENSIONS.y;

        let scale = scale_y.min(scale_x);

        ui_scale.0 = scale;
    }
}

fn t_slider_change(
    value_change: On<ValueChange<f32>>,
    mut commands: Commands,
    sliders: Query<&SliderValue>)
{
    if let Ok(slider_val) = sliders.get(value_change.source){
        if slider_val.0 == value_change.value{
            return;
        }

        commands
            .entity(value_change.source)
            .insert(SliderValue(value_change.value));

        commands.trigger(SliderValueChanged {
            entity: value_change.source,
            value: value_change.value,
        });
    }
}