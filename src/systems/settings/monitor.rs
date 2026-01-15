use crate::resources::{MonitorInfo, Monitors, RefreshRate};
use bevy::ecs::query::*;
use bevy::prelude::*;
use bevy::window::*;

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
    mut resource: ResMut<Monitors>,
    window: Query<&mut Window, With<PrimaryWindow>>,
){
    for (index, (entity, monitor)) in query.iter().enumerate() {

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

        if window.contains(entity){
            resource.selected_monitor = Some(index);
        }


        let mut refresh_rates: Vec<Box<RefreshRate>> = monitor.video_modes
            .iter()
            .map(|video_mode| {
                Box::new(RefreshRate(video_mode.refresh_rate_millihertz))
            })
            .collect();

        let mut resolutions: Vec<UIOption> = monitor.video_modes
            .iter()
            .map(|video_mode| { video_mode.physical_size })
            .collect();

        let mut bit_depths: Vec<u16> = monitor.video_modes
            .iter()
            .map(|video_mode| { video_mode.bit_depth })
            .collect();

        resolutions.sort_unstable_by_key(|r| (r.x, r.y));
        resolutions.dedup();

        refresh_rates.sort_unstable_by_key(|k| k.get_value::<u32>());
        refresh_rates.dedup_by_key(|k| k.get_value::<u32>());

        bit_depths.sort_unstable();
        bit_depths.dedup();


        resource.monitors.push(
            MonitorInfo {
                monitor_selection: MonitorSelection::Entity(entity),
                name,
                refresh_rates,
                resolutions,
                bit_depths
            }
        );
    }
}