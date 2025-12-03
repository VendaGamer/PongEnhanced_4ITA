use crate::components::{GameMode};
use crate::models::game::area::AreaShape;
use crate::models::game::fullscreen::ScreenMode;


#[derive(Clone, Eq, PartialEq, Hash)]
pub struct UIOption{
    pub text: &'static str,
    pub value: UIOptionValue,
}

impl UIOption {
    pub const fn int(text: &'static str, value: u32) -> Self{
        Self{
            text,
            value: UIOptionValue::Integer(value),
        }
    }

    pub const fn screen(text: &'static str, value: ScreenMode) -> Self{
        Self{
            text,
            value: UIOptionValue::Screen(value),
        }
    }

    pub const fn area(text: &'static str, value: AreaShape) -> Self{
        Self{
            text,
            value: UIOptionValue::AreaShape(value),
        }
    }

    pub const fn game_mode(text: &'static str, value: GameMode) -> Self{
        Self{
            text,
            value: UIOptionValue::GameMode(value),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum UIOptionValue
{
    Integer(u32),
    Screen(ScreenMode),
    AreaShape(AreaShape),
    GameMode(GameMode),
}
