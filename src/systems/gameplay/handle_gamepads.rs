use crate::bundles::Query;
use bevy::ecs::query::Spawned;
use bevy::prelude::{Commands, Entity, Gamepad};

pub fn check_connection(
    mut query: Query<(Entity, &Gamepad), Spawned>,
    mut commands: Commands
) {
    for (entity, gamepad) in query {

    }
}
