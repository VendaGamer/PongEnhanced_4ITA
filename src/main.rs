mod components;
mod resources;
mod systems;
mod bundles;
mod plugins;
mod utils;

use crate::plugins::GameCorePlugin;
use crate::resources::controls::PlayerAction;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

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
            .set(ImagePlugin::default_nearest()),

            RapierPhysicsPlugin::<NoUserData>::default(),
            GameCorePlugin,
            FpsOverlayPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        })
        .run();
}