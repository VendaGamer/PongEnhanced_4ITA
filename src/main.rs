mod components;
mod resources;
mod systems;
mod bundles;
mod plugins;
mod utils;
mod events;
mod models;
mod traits;

use crate::plugins::game_ui::GameUIPlugin;
use crate::plugins::GameCorePlugin;
use crate::resources::controls::PlayerAction;
use crate::resources::MenuAction;
use crate::systems::settings::persistence::load_settings;
use crate::utils::DEFAULT_FONT;
use avian2d::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::input_focus::directional_navigation::DirectionalNavigationPlugin;
use bevy::input_focus::InputDispatchPlugin;
use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::{PresentMode, WindowResolution};
use bevy_tween::DefaultTweenPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    let mut app = App::new();

    let settings = load_settings();
    let mut window_resolution: WindowResolution = WindowResolution::default();

    window_resolution.set(settings.window_resolution.x as f32, settings.window_resolution.y as f32);

    let video_mode = settings.video_mode;

    app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        mode: video_mode,
                        resolution: window_resolution,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
            .disable::<bevy::pbr::PbrPlugin>(),
            FpsOverlayPlugin{
                config: FpsOverlayConfig {
                    enabled: settings.show_fps,
                    text_color: Srgba::rgb(1.0, 0.73, 0.23).into(),
                    frame_time_graph_config: FrameTimeGraphConfig{
                        enabled: false,
                        ..default()
                    },
                    ..default()
                },
            },
            PhysicsPlugins::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuAction>::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            DefaultTweenPlugins,
            DirectionalNavigationPlugin,

            // my plugins
            GameCorePlugin,
            GameUIPlugin,
        ))
        .insert_resource(settings);

    let world = app.world_mut();

    world.resource_mut::<Assets<_>>()
        .insert(AssetId::default(), Font::try_from_bytes(DEFAULT_FONT.into())
        .unwrap())
        .expect("UNABLE TO LOAD FONT");


    app.run();
}
