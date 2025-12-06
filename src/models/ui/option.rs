use crate::models::game::area::AreaShape;
use crate::models::game::fullscreen::ScreenMode;
use crate::models::game::gameplay::GameMode;
use derive_more::From;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct UIOption {
    pub text: &'static str,
    pub value: UIOptionValue,
}

impl UIOption {
    pub fn new(text: &'static str, value: UIOptionValue) -> Self {
        Self {
            text,
            value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
pub enum UIOptionValue
{
    Integer(u32),
    Screen(ScreenMode),
    AreaShape(AreaShape),
    GameMode(GameMode),
}