#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum ScreenMode {
    ExclusiveFullScreen,
    #[default]
    BorderlessFullScreen,
    Windowed,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}