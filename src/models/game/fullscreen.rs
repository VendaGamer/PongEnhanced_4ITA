#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScreenMode {
    ExclusiveFullScreen,
    FullScreen,
    Windowed,
}