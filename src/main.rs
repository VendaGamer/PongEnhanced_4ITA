mod bundles;
mod components;
mod events;
mod models;
mod networking;
mod plugins;
mod resources;
mod systems;
mod traits;
mod utils;

use crate::networking::shared::GameNetworking;
use crate::plugins::game_ui::GameUIPlugin;
use crate::plugins::GameCorePlugin;
use crate::resources::MenuAction;
use crate::systems::settings::persistence::load_settings;
use crate::utils::DEFAULT_FONT;
use avian2d::prelude::*;
use bevy::input_focus::directional_navigation::DirectionalNavigationPlugin;
use bevy::input_focus::InputDispatchPlugin;
use bevy::prelude::*;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::WindowResolution;
use bevy_tween::DefaultTweenPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    let mut app = App::new();

    let settings = load_settings();
    let mut window_resolution: WindowResolution = WindowResolution::default();

    if let Some(res) = settings.window_resolution {
        window_resolution.set_physical_resolution(res.x, res.y);
    }

    let video_mode = settings.window_mode;

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong Enhanced".into(),
                    present_mode: settings.vsync,
                    resizable: false,
                    mode: video_mode,
                    resolution: window_resolution,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        PhysicsPlugins::default()
            .build()
            .disable::<PhysicsTransformPlugin>()
            .disable::<PhysicsInterpolationPlugin>(),
        InputManagerPlugin::<MenuAction>::default(),
        UiWidgetsPlugins,
        InputDispatchPlugin,
        DefaultTweenPlugins,
        DirectionalNavigationPlugin,
        
        // my plugins
        GameCorePlugin,
        GameUIPlugin,
        GameNetworking,
    ))
    .insert_resource(settings);

    let world = app.world_mut();

    world
        .resource_mut::<Assets<_>>()
        .insert(
            AssetId::default(),
            Font::try_from_bytes(DEFAULT_FONT.into()).unwrap(),
        )
        .expect("UNABLE TO LOAD FONT");

    app.run();
}
