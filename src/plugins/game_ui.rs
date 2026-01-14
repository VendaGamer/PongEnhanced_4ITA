use crate::bundles::{App, Commands, On, Plugin, Update};
use crate::events::widgets::SliderValueChanged;
use crate::systems::widgets::*;
use bevy::prelude::Query;
use bevy::ui_widgets::{SliderValue, ValueChange};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                u_ui_hover_light,
                u_slider_visuals,
                t_button_press,
                ))
            .add_observer(t_slider_change)
            .add_observer(update_selector);
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