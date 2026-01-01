use crate::components::Player;
use crate::resources::PlayerAction;
use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent};
use bevy::prelude::{MessageReader, Query, With};
use leafwing_input_manager::input_map::InputMap;

pub fn check_connection(
    mut events: MessageReader<GamepadConnectionEvent>,
    mut bindings: Query<&InputMap<PlayerAction>, With<Player>>,
) {
    for ev in events.read() {

        if let GamepadConnection::Connected{ .. } = &ev.connection {

            println!("Connected Gamepad");

        }

    }
}
