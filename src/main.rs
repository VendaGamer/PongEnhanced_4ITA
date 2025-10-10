pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::systems::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            GameplayPlugin
        ))
        .run();
}