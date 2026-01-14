use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
pub enum ScreenMode {
    ExclusiveFullScreen,
    #[default]
    BorderlessFullScreen,
    Windowed,
}