use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::models::game::area::{PlayerInfo, Players};
use crate::resources::controls::MenuAction;
use crate::resources::navigation::UISelection;
use crate::resources::GameConfig;
use crate::systems::menu::MenuSpawnCommandsExt;
use crate::systems::navigation::{sync_selection_to_ui, ui_navigation};
use crate::systems::selectors::{handle_selector_navigation, update_selector_text};
use crate::systems::*;
use crate::utils::FIXED_DIMENSIONS;
use bevy::window::{Monitor, WindowResized};

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
                handle_ui_scaling
            ))
            .add_systems(Startup, (
                //setup,
                setup_common,
                print_available_resolutions
            ))
            .insert_resource(UISelection::default())
            .insert_resource(GameConfig::default())
            .insert_resource(Players::default());
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
        let info = PlayerInfo{
            entity,
            name: format!("Player {}", i + 1),
        };
        players.players.push(info);
    }

    commands.spawn(MenuAction::input_map());
    commands.spawn_main_menu();
}

fn print_available_resolutions(
    mut monitors: Query<(Entity, &mut Monitor)>,
) {
    for (entity, mut monitor) in monitors.iter_mut() {
        println!("\n=== Monitor Entity: {:?} ===", entity);

        if let Some(name) = &monitor.name {
            dbg!("Name: {}", name);
        } else {
            dbg!("Name: <unnamed>");
        }

        dbg!("Physical size: {}x{} px",
             monitor.physical_width,
             monitor.physical_height
        );

        dbg!("Position: ({}, {})",
             monitor.physical_position.x,
             monitor.physical_position.y
        );

        monitor.scale_factor = 2.0;

        dbg!("Scale factor: {}", monitor.scale_factor);

        dbg!("\nSupported video modes:");
        for (i, mode) in monitor.video_modes.iter().enumerate() {
            dbg!(
                "  {}. {}x{} @ {:.2}Hz (Bit depth: {})",
                i + 1,
                monitor.physical_width,
                monitor.physical_height,
                mode.refresh_rate_millihertz as f32 / 1000.0,
                mode.bit_depth
            );
        }

        dbg!("Total video modes: {}", monitor.video_modes.len());
    }
}