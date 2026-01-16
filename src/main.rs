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
use crate::utils::DEFAULT_FONT;
use avian2d::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::input_focus::InputDispatchPlugin;
use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::PresentMode;
use bevy_tween::DefaultTweenPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::plugins::state_persistence::GameStatePersistencePlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
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
            UiWidgetsPlugins,
            InputDispatchPlugin,
            DefaultTweenPlugins,

            // my plugins
            GameCorePlugin,
            GameUIPlugin,
            GameStatePersistencePlugin
        ));

    let world = app.world_mut();

    world.resource_mut::<Assets<_>>()
        .insert(AssetId::default(), Font::try_from_bytes(DEFAULT_FONT.into())
        .unwrap())
        .expect("UNABLE TO LOAD FONT");
    
    
    app.run();
}
