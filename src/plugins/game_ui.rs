use crate::bundles::{App, Commands, MessageReader, On, Plugin, ResMut, UiScale, Update};
use crate::events::gameplay::UINavigated;
use crate::events::widgets::{SliderValueChanged, TextInputSubmitted};
use crate::systems::widgets::*;
use crate::utils::FIXED_DIMENSIONS;
use bevy::ecs::relationship::Relationship;
use bevy::input_focus::directional_navigation::DirectionalNavigation;
use bevy::prelude::{ChildOf, Display, Entity, Node, Query};
use bevy::ui_widgets::{SliderValue, ValueChange};
use bevy::window::WindowResized;
use bevy_simple_text_input::TextInputSubmitMessage;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                u_ui_hover_light,
                u_slider_visuals,
                t_button_press,
                handle_ui_scaling,
                u_highlight_focused_element,
                t_navigate_element,
                u_button_press,
                t_input_submit,
            ),
        )
        .add_observer(handle_invisible_nav)
        .add_observer(t_slider_change)
        .add_observer(update_selector);
    }
}

#[inline]
fn handle_invisible_nav(
    event: On<UINavigated>,
    nodes: Query<(&Node, Option<&ChildOf>)>,
    nav: DirectionalNavigation
) {
    handle_invisible_nav_core(*event, nodes, nav);
}
#[inline]
fn t_input_submit(
    mut messages: MessageReader<TextInputSubmitMessage>,
    mut commands: Commands,
) {
    for message in messages.read() {
        commands.trigger(TextInputSubmitted{
            entity: message.entity,
            value: message.value.clone(),
        })
    }
}

fn handle_invisible_nav_core(
    event: UINavigated,
    nodes: Query<(&Node, Option<&ChildOf>)>,
    mut nav: DirectionalNavigation
){
    if let Ok((node, parent)) = nodes.get(event.entity) {
        if is_display_none(&nodes, node, parent) {
            match nav.navigate(event.direction) {
                Ok(entity) => {
                    handle_invisible_nav_core(
                        UINavigated{
                            direction: event.direction,
                            entity,
                        },
                        nodes,
                        nav
                    );
                },
                _ => {}
            }
        }
    }
}

fn is_display_none(
    query: &Query<(&Node, Option<&ChildOf>)>,
    node: &Node,
    parent: Option<&ChildOf>
) -> bool {
    if matches!(node.display, Display::None) {
        return true;
    }

    if let Some(parent) = parent {
        if let Ok((parent_node, grandparent)) = query.get(parent.get()) {
            return is_display_none(query, parent_node, grandparent);
        }
    }

    false
}

fn handle_ui_scaling(mut ui_scale: ResMut<UiScale>, mut resized: MessageReader<WindowResized>) {
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
    sliders: Query<&SliderValue>,
) {
    if let Ok(slider_val) = sliders.get(value_change.source) {
        if slider_val.0 == value_change.value {
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
