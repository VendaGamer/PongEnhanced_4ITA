use crate::bundles::KeyCode;
use bevy::prelude::GamepadButton;
use bevy::reflect::Reflect;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(Axis)]
    Move,
    #[actionlike(Axis)]
    Tilt,
    Dash,
    Push,
    Pause,
}

#[derive(Actionlike, Clone, Copy, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum MenuAction {
    Confirm,
    Cancel,
    #[actionlike(DualAxis)]
    Navigate
}

impl MenuAction{
    pub fn input_map() -> InputMap<Self>{
        let mut map = InputMap::default();

        map.insert(MenuAction::Confirm, KeyCode::Enter);
        map.insert(MenuAction::Confirm, KeyCode::Space);
        map.insert(MenuAction::Cancel, KeyCode::Escape);

        map.insert(MenuAction::Confirm, GamepadButton::South);
        map.insert(MenuAction::Cancel, GamepadButton::East);


        map.insert_dual_axis(
            MenuAction::Navigate,
            VirtualDPad::wasd(),
        );
        map.insert_dual_axis(
            MenuAction::Navigate,
            VirtualDPad::arrow_keys(),
        );
        
        map.insert_dual_axis(
            MenuAction::Navigate,
            GamepadStick::LEFT,
        );
        map.insert_dual_axis(
            MenuAction::Navigate, 
            VirtualDPad::dpad(),
        );

        map
    }
}