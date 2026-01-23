use std::fmt::Write;
use crate::components::ui::{SourceHandle, UIOptionProvider, UIOptionString};
use bevy::window::PresentMode;

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

pub const FPS_SHOW_OPTIONS: SourceHandle<dyn UIOptionProvider> =
SourceHandle::Static(&FPS_SHOW_OPTIONS_RAW);

pub const FPS_SHOW_OPTIONS_RAW: [ShowFPSMode; 3] = [
    ShowFPSMode::No,
    ShowFPSMode::Yes,
    ShowFPSMode::Detailed
];

impl UIOptionString for ShowFPSMode {
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match *self {
            ShowFPSMode::Detailed => "Detailed",
            ShowFPSMode::Yes => "Yes",
            ShowFPSMode::No => "No",
        };

        string.push_str(s);
    }
}

pub enum ShowFPSMode {
    Detailed,
    Yes,
    No,
}


impl<T: Write> UIOptionString for T {
    #[inline]
    fn push_ui_option_string(&self, string: &mut String) {
        _ = string.write_str(self);
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