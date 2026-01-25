use crate::bundles::{Entity, GamepadAxis};
use crate::models::game::area::PlayerID;
use crate::resources::controls::PlayerAction;
use bevy::prelude::{Component, GamepadButton, KeyCode};
use leafwing_input_manager::prelude::{InputMap, VirtualAxis};

#[derive(Component)]
pub struct Player {
    pub id: PlayerID,
}


impl Player {

    pub fn get_gamepad_input_map(gamepad: Entity) -> InputMap<PlayerAction> {

        let mut map = InputMap::new([
            (PlayerAction::Dash, GamepadButton::LeftTrigger2),
            (PlayerAction::Push, GamepadButton::RightTrigger2),
            (PlayerAction::Pause, GamepadButton::Start),
        ]).with_gamepad(gamepad);

        map.insert_axis(PlayerAction::Move, GamepadAxis::LeftStickX)
            .insert_axis(PlayerAction::Move, GamepadAxis::LeftStickY)
            .insert_axis(PlayerAction::Tilt, GamepadAxis::RightStickX)
            .insert_axis(PlayerAction::Tilt, GamepadAxis::RightStickY);

        map
    }


    pub fn get_default_input_map(id: u8) -> InputMap<PlayerAction> {
        match id {
            1 => {
                let mut map = InputMap::new([
                    (PlayerAction::Dash, KeyCode::ControlLeft),
                    (PlayerAction::Push, KeyCode::Space),
                    (PlayerAction::Pause, KeyCode::Escape),
                ]);

                map.insert_axis(PlayerAction::Move, VirtualAxis::ws())
                    .insert_axis(PlayerAction::Tilt, VirtualAxis::ad());

                map
            },
            2 =>{
                let mut map = InputMap::new([
                    (PlayerAction::Dash, KeyCode::ControlRight),
                    (PlayerAction::Push, KeyCode::Enter),
                    (PlayerAction::Pause, KeyCode::End),
                ]);

                map.insert_axis(PlayerAction::Move, VirtualAxis::vertical_arrow_keys())
                    .insert_axis(PlayerAction::Tilt, VirtualAxis::horizontal_arrow_keys());

                map
            },
            _ => panic!("Invalid id: {}", id),
        }
    }
}