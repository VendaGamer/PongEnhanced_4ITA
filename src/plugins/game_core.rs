use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::models::game::area::{PlayerInfo, Players};
use crate::resources::controls::MenuAction;
use crate::resources::navigation::{NavigationState, UISelection};
use crate::resources::GameConfig;
use crate::systems::menu::MenuSpawnCommandsExt;
use crate::systems::navigation::{sync_selection_to_ui, ui_navigation};
use crate::systems::selectors::{handle_selector_navigation, update_selector_text};
use crate::systems::*;
use crate::utils::FIXED_DIMENSIONS;
use bevy::ui_widgets::slider_self_update;
use bevy::window::{Monitor, WindowResized};
use crate::systems::widgets::update_slider_visuals;

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
                detect_button_press,
                handle_ui_hover_light,
                ui_navigation,
                sync_selection_to_ui,
                update_selector_text,
                handle_selector_navigation,
                handle_ui_scaling,
                update_slider_visuals
            ))
            .add_systems(Startup, (
                //setup,
                setup_common,
                print_available_resolutions
            ))
            .add_observer(slider_self_update)
            .insert_resource(UISelection::default())
            .insert_resource(GameConfig::default())
            .insert_resource(Players::default())
            .insert_resource(NavigationState::default());
    }
}

fn handle_ui_scaling(
    mut ui_scale: ResMut<UiScale>,
    mut resized: MessageReader<WindowResized>)
{
    for event in resized.read() {

        let scale_x = event.width / FIXED_DIMENSIONS.x;
        let scale_y = event.height / FIXED_DIMENSIONS.y;

        let scale = scale_y.min(scale_x);

        ui_scale.0 = scale;
    }
}

fn setup_common(
    mut commands: Commands,
    mut players: ResMut<Players>,
) {
    commands.spawn(CameraBundle::default());

    for i in 0..4{
        let entity = commands.spawn(PlayerBundle::new(i)).id();
        let info = PlayerInfo {
            entity,
            name: format!("Player {}", i + 1),
        };
        players.players.push(info);
    }

    commands.spawn(MenuAction::input_map());
    commands.spawn_main_menu();
}

fn print_available_resolutions(
    monitors: Query<&Monitor>,
) {
    for monitor in monitors.iter() {
       dbg!(monitor);
    }
}