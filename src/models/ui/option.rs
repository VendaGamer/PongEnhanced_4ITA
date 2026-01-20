use crate::components::ui::{SourceHandle, UIOptionProvider, UIOptionString};
use bevy::window::PresentMode;


pub const VSYNC_OPTIONS: SourceHandle<dyn UIOptionProvider> =
SourceHandle::Static
(&[
    PresentMode::AutoNoVsync,
    PresentMode::AutoVsync
]);

impl UIOptionString for PresentMode {
    fn push_ui_option_string(&self, string: &mut String) {

        let s = match *self {
            PresentMode::AutoVsync => "Vsync On",
            PresentMode::Fifo => "Vsync On",
            PresentMode::FifoRelaxed => "Adaptive Vsync",
            PresentMode::Immediate => "Vsync Off",
            PresentMode::Mailbox => "Fast Vsync",
            PresentMode::AutoNoVsync => "Vsync Off",
        };

        string.push_str(s);
    }
}