use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Speedup,
    Dash,
    Push,
    Pause
}

#[derive(Actionlike, Clone, Copy, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum MenuAction {
    Confirm,
    Cancel,
    NavigateY,
    NavigateX,
}
