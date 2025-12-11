#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ScreenMode {
    ExclusiveFullScreen,
    FullScreen,
    Windowed,
}