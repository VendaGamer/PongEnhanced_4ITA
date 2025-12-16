use crate::resources::controls::PlayerAction;
use bevy::prelude::{Component, KeyCode};
use leafwing_input_manager::prelude::InputMap;

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

impl Player {
    pub fn get_default_input_map(id: u8) -> InputMap<PlayerAction> {
        match id {
            1 => InputMap::new([
                (PlayerAction::Up, KeyCode::KeyW),
                (PlayerAction::Down, KeyCode::KeyS),
                (PlayerAction::Right, KeyCode::KeyD),
                (PlayerAction::Left, KeyCode::KeyA),
                (PlayerAction::Dash, KeyCode::ControlLeft),
                (PlayerAction::Push, KeyCode::Space),
                (PlayerAction::Pause, KeyCode::Escape),
                (PlayerAction::Speedup, KeyCode::ShiftLeft),
            ]),
            2 => InputMap::new([
                (PlayerAction::Up, KeyCode::ArrowUp),
                (PlayerAction::Down, KeyCode::ArrowDown),
                (PlayerAction::Right, KeyCode::ArrowRight),
                (PlayerAction::Left, KeyCode::ArrowLeft),
                (PlayerAction::Dash, KeyCode::ControlRight),
                (PlayerAction::Push, KeyCode::Enter),
                (PlayerAction::Pause, KeyCode::End),
                (PlayerAction::Speedup, KeyCode::ShiftRight),
            ]),
            3 => InputMap::new([
                (PlayerAction::Up, KeyCode::KeyI),
                (PlayerAction::Down, KeyCode::KeyK),
                (PlayerAction::Right, KeyCode::KeyL),
                (PlayerAction::Left, KeyCode::KeyJ),
                (PlayerAction::Dash, KeyCode::KeyO),
                (PlayerAction::Push, KeyCode::KeyU),
                (PlayerAction::Pause, KeyCode::KeyP),
                (PlayerAction::Speedup, KeyCode::KeyB),
            ]),
            4 => InputMap::new([
                (PlayerAction::Up, KeyCode::Numpad8),
                (PlayerAction::Down, KeyCode::Numpad5),
                (PlayerAction::Right, KeyCode::Numpad6),
                (PlayerAction::Left, KeyCode::Numpad4),
                (PlayerAction::Dash, KeyCode::NumpadComma),
                (PlayerAction::Push, KeyCode::NumpadEnter),
                (PlayerAction::Pause, KeyCode::NumpadDivide),
                (PlayerAction::Speedup, KeyCode::Numpad0),
            ]),
            _ => InputMap::default(),
        }
    }
}