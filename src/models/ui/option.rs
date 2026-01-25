use crate::components::ui::{SourceHandle, UIOptionProvider, UIOptionString};
use bevy::window::PresentMode;
use std::fmt::Write;

pub const VSYNC_OPTIONS: SourceHandle<dyn UIOptionProvider> =
SourceHandle::Static(&VSYNC_OPTIONS_RAW);


pub const VSYNC_OPTIONS_RAW: [PresentMode; 2] = [
    PresentMode::AutoNoVsync,
    PresentMode::AutoVsync
];

pub const FPS_LOCK_OPTIONS: SourceHandle<dyn UIOptionProvider> =
SourceHandle::Static(&FPS_LOCK_OPTIONS_RAW);

pub const FPS_LOCK_OPTIONS_RAW: [u16; 3] = [
    30,
    60,
    120
];

impl UIOptionString for u16 {
    #[inline]
    fn push_ui_option_string(&self, string: &mut String) {
        write!(string, "{}", self).unwrap();
    }
}


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