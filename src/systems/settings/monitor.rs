use std::sync::Arc;
use crate::resources::{BitDepth, MonitorInfo, Monitors, RefreshRate, Resolution};
use bevy::ecs::query::*;
use bevy::prelude::*;
use bevy::window::*;
use crate::components::ui::UIOptionValue;

#[cfg(windows)]
fn get_monitor_name_windows(device_path: &str) -> Option<String> {
    use windows::Win32::Graphics::Gdi::*;
    use windows::core::PWSTR;

    let mut display_device = DISPLAY_DEVICEW::default();
    display_device.cb = size_of::<DISPLAY_DEVICEW>() as u32;

    let wide: Vec<u16> = device_path.encode_utf16().chain(Some(0)).collect();

    unsafe {
        if EnumDisplayDevicesW(
            PWSTR(wide.as_ptr() as *mut _),
            0,
            &mut display_device,
            0,
        )
            .as_bool()
        {
            let mut idx: usize = 0;

            for char in display_device.DeviceString {
                if char == 0{
                    break;
                }
                idx+=1;
            }

            return String::from_utf16(&display_device.DeviceString[..idx]).ok();
        }
    }

    None
}

pub fn on_spawn_monitors(
    query: Query<(Entity, &Monitor), Spawned>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
){
    let mut info: Vec<Box<MonitorInfo>> = Vec::new();
    let current_monitor_index :usize = 0;
    let primary_window = window.single().expect("Couldn't get primary window");

    let selected_monitor = match primary_window.mode{
        WindowMode::BorderlessFullscreen(monitor) => Some(monitor),
        WindowMode::Fullscreen(monitor, _) => Some(monitor),
        _ => None,
    };

    for (index, (entity, monitor)) in query.iter().enumerate() {

        let selection = MonitorSelection::Entity(entity);
        let name = if let Some(real_name) = monitor.name.clone(){

            #[cfg(windows)]
            {
                if let Some(name) = get_monitor_name_windows(&real_name) {
                    name
                }else{
                    real_name
                }
            }
            #[cfg(not(windows))]
            {
                real_name
            }

        }else{
            format!("Monitor {}", index + 1)
        };


        if let Some(current_monitor) = selected_monitor {
            if current_monitor.eq(&selection){

            }
        }



        let mut refresh_rates: Vec<Box<RefreshRate>> = monitor.video_modes
            .iter()
            .map(|video_mode|
                Box::new(video_mode.refresh_rate_millihertz.into())
            )
            .collect();

        let mut resolutions: Vec<Box<Resolution>> = monitor.video_modes
            .iter()
            .map(|video_mode|
                 Box::new(video_mode.physical_size.into())
            )
            .collect();

        let mut bit_depths: Vec<Box<BitDepth>> = monitor.video_modes
            .iter()
            .map(|video_mode|
                 Box::new(video_mode.bit_depth.into())
            )
            .collect();

        resolutions.sort_unstable_by_key(|r| (r.0.x, r.0.y));
        resolutions.dedup();

        refresh_rates.sort_unstable();
        refresh_rates.dedup();

        bit_depths.sort_unstable();
        bit_depths.dedup();

        let bit_depth = bit_depths.iter().map(|x| x.0).max().expect("No bit depths");
        let refresh_rate = monitor.refresh_rate_millihertz.unwrap_or(
            refresh_rates.iter().map(|x| x.0).max().expect("No Refresh rates")
        );

        info.push(Box::new(
            MonitorInfo{
                monitor_selection: selection,
                name,
                refresh_rates: Arc::new(refresh_rates),
                resolutions: Arc::new(resolutions),
                bit_depths: Arc::new(bit_depths),
                native_mode: VideoMode {
                    bit_depth,
                    refresh_rate_millihertz: refresh_rate,
                    physical_size: monitor.physical_size(),
                }
            }
        ));
    }

    commands.insert_resource(
        Monitors{
            monitors: Arc::new(info),
            selected_monitor: current_monitor_index,
        }
    );

}