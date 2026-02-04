use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::components::Player;
use crate::models::game::area::LocalPlayerID;
use crate::resources::controls::MenuAction;
use crate::resources::GameModeConfig;
use crate::systems::menu::{spawn_m_main, u_join_in, u_settings_visibility};
use crate::systems::selectors::update_selector_text;
use crate::systems::settings::monitor::on_spawn_monitors;
use crate::systems::*;
use avian2d::prelude::Gravity;
use bevy::ecs::query::Spawned;
use bevy::input_focus::directional_navigation::DirectionalNavigationMap;
use bevy::input_focus::InputFocusVisible;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                u_move_paddle_i,
                check_connection,
                maintain_ball_speed,
                update_score_ui,
                update_selector_text,
                u_join_in,
                u_spawned_gamepads,
                u_tilt_i,
                u_settings_visibility
            ),
        )
        .add_systems(Startup, (setup_common,))
        .add_systems(PostStartup, (on_spawn_monitors,))
        .add_observer(paddle_hit_dynamics)
        .add_observer(t_ball_events)
        .insert_resource(GameModeConfig::default())
        .insert_resource(Gravity::ZERO)
        .insert_resource(InputFocusVisible(false));
    }
}

fn u_spawned_gamepads(query: Query<Entity, (Spawned, With<Gamepad>)>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).observe(on_despawn_gamepad);
        commands.spawn(PlayerBundle::new(LocalPlayerID::Gamepad(entity)));
    }
}

fn on_despawn_gamepad(
    despawn: On<Despawn>,
    mut players: Query<(Entity, &mut Player)>,
    mut commands: Commands,
) {
    for (player_entity, player) in players.iter_mut() {
        if let LocalPlayerID::Gamepad(entity) = player.id.local() {
            if entity == despawn.entity {
                commands.entity(player_entity).despawn();
            }
        }
    }
}

fn setup_common(mut commands: Commands, mut map: ResMut<DirectionalNavigationMap>) {
    commands.spawn(CameraBundle::default());

    for i in 1..=2 {
        commands.spawn(PlayerBundle::new(LocalPlayerID::Keyboard(i)));
    }

    commands.spawn(MenuAction::input_map());
    spawn_m_main(&mut commands, map.as_mut());
}
