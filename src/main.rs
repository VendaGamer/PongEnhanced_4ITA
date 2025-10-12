pub mod components;
pub mod resources;
pub mod systems;
pub mod bundles;
pub mod plugins;

use crate::plugins::GameCorePlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;
pub use components::*;

fn main() {
    App::new()
        .add_plugins((

            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest()),

            RapierPhysicsPlugin::<NoUserData>::default(),
            GameCorePlugin
        ))
        .insert_resource(TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        })
        .run();
}