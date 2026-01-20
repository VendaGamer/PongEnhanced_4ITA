use bevy::ecs::query::Spawned;
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::components::Player;
use crate::models::game::area::{PlayerID};
use crate::resources::controls::MenuAction;
use crate::resources::{GameModeConfig, GameSettings, Monitors};
use crate::systems::menu::{m_main, u_join_in};
use crate::systems::selectors::update_selector_text;
use crate::systems::settings::monitor::on_spawn_monitors;
use crate::systems::*;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_paddle,
                check_connection,
                maintain_ball_speed,
                paddle_hit_dynamics,
                update_score_ui,
                update_selector_text,
                u_join_in,
                u_spawned_gamepads
            ))
            .add_systems(Startup, (
                setup_common,
            ))
            .add_systems(PostStartup, (
                on_spawn_monitors,
            ))
            .insert_resource(GameModeConfig::default());
    }
}


fn u_spawned_gamepads(
    query: Query<Entity, With<(Gamepad, Spawned)>>,
    mut commands: Commands
) {

    for entity in query.iter() {

        commands.entity(entity).observe(on_despawn_gamepad);
    }
}

fn on_despawn_gamepad(
    despawn :On<Despawn>,
    mut players: Query<&mut Player>)
{
    for player in players.iter_mut() {

        if let PlayerID::Gamepad(entity) = player.id{

        }
    }
}

fn setup_common(
    mut commands: Commands,
) {
    commands.spawn(CameraBundle::default());

    for i in 1..=2 {
        commands.spawn(PlayerBundle::new_keyboard(i));
    }

    commands.spawn(MenuAction::input_map());
    commands.spawn(m_main());
}