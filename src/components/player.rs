use crate::resources::controls::PlayerAction;
use bevy::prelude::{Component, Entity, KeyCode};
use leafwing_input_manager::prelude::InputMap;

#[derive(Component)]
pub struct Player {
    pub id: u8,
    pub team: Entity,
    pub name: String,
}

#[derive(Component)]
pub struct ControlledPaddle(pub Entity);

impl Player{
    pub fn get_default_input_map(player : &Self) -> InputMap<PlayerAction> {
        match player.id {
            1 => InputMap::new([
                (PlayerAction::Up, KeyCode::KeyW),
                (PlayerAction::Down, KeyCode::KeyS),
                (PlayerAction::Right, KeyCode::KeyD),
                (PlayerAction::Left, KeyCode::KeyA),
                (PlayerAction::Dash, KeyCode::ShiftLeft),
                (PlayerAction::Push, KeyCode::Space),
            ]),
            2 => InputMap::new([
                (PlayerAction::Up, KeyCode::ArrowUp),
                (PlayerAction::Down, KeyCode::ArrowDown),
                (PlayerAction::Right, KeyCode::ArrowRight),
                (PlayerAction::Left, KeyCode::ArrowLeft),
                (PlayerAction::Dash, KeyCode::ControlRight),
                (PlayerAction::Push, KeyCode::Enter),
            ]),
            3 => InputMap::new([
                (PlayerAction::Up, KeyCode::KeyI),
                (PlayerAction::Down, KeyCode::KeyK),
                (PlayerAction::Right, KeyCode::KeyL),
                (PlayerAction::Left, KeyCode::KeyJ),
                (PlayerAction::Dash, KeyCode::KeyO),
                (PlayerAction::Push, KeyCode::KeyU),
            ]),
            4 => InputMap::new([
                (PlayerAction::Up, KeyCode::Numpad8),
                (PlayerAction::Down, KeyCode::Numpad5),
                (PlayerAction::Right, KeyCode::Numpad6),
                (PlayerAction::Left, KeyCode::Numpad4),
                (PlayerAction::Dash, KeyCode::Numpad0),
                (PlayerAction::Push, KeyCode::NumpadEnter),
            ]),
            _ => InputMap::default(),
        }
    }
}