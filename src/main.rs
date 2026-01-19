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
use bevy::input_focus::InputDispatchPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{Backends, PowerPreference, RenderCreation, WgpuSettings, WgpuSettingsPriority};
use bevy::render::RenderPlugin;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::PresentMode;
use bevy_tween::DefaultTweenPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use wgpu_types::Limits;

fn main() {
    let mut app = App::new();

    let settings = load_settings();

    app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        mode: settings.video_mode,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
            .disable::<bevy::pbr::PbrPlugin>()
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(
                    WgpuSettings {
                        backends: Some(Backends::all()),
                        priority: WgpuSettingsPriority::Functionality,
                        power_preference: PowerPreference::HighPerformance,
                        features: WgpuFeatures::default()
                            .difference(WgpuFeatures::VERTEX_WRITABLE_STORAGE),
                        limits: Limits {
                            max_storage_buffers_per_shader_stage: 10,
                            ..default()
                        },
                        ..default()
                    }
                ),
                ..default()
            }),
            PhysicsPlugins::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuAction>::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            DefaultTweenPlugins,

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
