mod components;
mod resources;
mod systems;
mod bundles;
mod plugins;
mod utils;
mod events;
mod models;

use crate::plugins::GameCorePlugin;
use crate::resources::controls::PlayerAction;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::window::PresentMode;
use avian2d::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::resources::MenuAction;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::all()),
                    ..default()
                }),
                ..default()
            }),

            PhysicsPlugins::default(),
            GameCorePlugin,
            FpsOverlayPlugin{
                config: FpsOverlayConfig{
                    enabled: true,
                    text_color: Srgba::rgb(1.0, 0.73, 0.23).into(),
                    frame_time_graph_config: FrameTimeGraphConfig{
                        enabled: false,
                        ..default()
                    },
                    ..default()
                },
            },
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuAction>::default(),
            UiWidgetsPlugins
        ))
        .run();
}
