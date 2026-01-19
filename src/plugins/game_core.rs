use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::models::game::area::{PlayerID, PlayerInfo, Players};
use crate::resources::controls::MenuAction;
use crate::resources::{GameModeConfig, GameSettings, Monitors};
use crate::systems::menu::m_main;
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
            ))
            .add_systems(Startup, (
                setup_common,
            ))
            .add_systems(PostStartup, (
                on_spawn_monitors,
            ))
            .insert_resource(GameModeConfig::default())
            .insert_resource(Players::default());
    }
}

fn setup_common(
    mut commands: Commands,
    mut players: ResMut<Players>
) {
    commands.spawn(CameraBundle::default());

    for i in 0..4{
        let entity = commands.spawn(PlayerBundle::new(i)).id();
        let info = PlayerInfo {
            id: PlayerID(i),
            name: format!("Player {}", i + 1),
        };
        players.players.push(info);
    }

    commands.spawn(MenuAction::input_map());
    commands.spawn(m_main());
}