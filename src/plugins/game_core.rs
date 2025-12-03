use crate::bundles::area::AreaBundle;
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::components::ui::{Menu, ScoreText};
use crate::components::*;
use crate::resources::controls::MenuAction;
use crate::resources::navigation::UISelection;
use crate::systems::navigation::{sync_selection_to_ui, ui_navigation};
use crate::systems::selectors::{handle_selector_navigation, update_selector_text};
use crate::systems::*;
use crate::utils::screen::BALL_RADIUS;
use crate::utils::FIXED_DIMENSIONS;
use bevy::window::{Monitor, WindowResized};
use crate::models::game::area::AreaShape;
use crate::resources::GameConfig;
use crate::systems::menu::MenuSpawnCommandsExt;

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
            .insert_resource(UISelection::default());
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
) {
    commands.spawn(CameraBundle::default());

    commands.spawn(PlayerBundle::new(
        Player {
            id: 1,
            team: None,
            name: "Player 1".into(),
        }
    ));

    commands.spawn(PlayerBundle::new(
        Player {
            id: 2,
            team: None,
            name: "Player 2".into(),
        }
    ));

    commands.spawn(PlayerBundle::new(
        Player {
            id: 3,
            team: None,
            name: "Player 3".into(),
        }
    ));

    commands.spawn(PlayerBundle::new(
        Player {
            id: 4,
            team: None,
            name: "Player 4".into(),
        }
    ));


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

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    menu: Query<Entity, With<Menu>>,
) {

    commands.entity(menu.single().expect("No menu")).despawn();

    let team1 = commands.spawn(Team {
        name: "Team Left".into(),
        current_score: 0,
    }).id();

    let team2 = commands.spawn(Team {
        name: "Team Right".into(),
        current_score: 0,
    }).id();

    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Start,
        padding: UiRect::all(Val::Px(50.0)),
        ..default()
    }).with_children(|parent| {


            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ScoreText { team: team2 },
            ));


            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ScoreText { team: team1 },
            ));
    });

    
    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-300.0, 300.0),
        BALL_RADIUS
    )).observe(handle_scoring);

    AreaBundle::spawn(AreaShape::TwoSide([team1, team2]), &mut commands);

    const SEGMENT_HEIGHT: f32 = 20.0;
    const GAP_HEIGHT: f32 = 15.0;
    const HALF_HEIGHT: f32 = 360.0;

    let mut y_pos = -HALF_HEIGHT + SEGMENT_HEIGHT / 2.0;
    while y_pos < HALF_HEIGHT {
        commands.spawn(DivisionLineBundle::new(&mut meshes, &mut materials))
            .insert(Transform::from_translation(Vec3::new(0.0, y_pos, 0.0)));
        y_pos += SEGMENT_HEIGHT + GAP_HEIGHT;
    }
}


fn update_score_ui(
    teams: Query<&Team>,
    mut score_texts: Query<(&mut Text, &ScoreText)>,
) {
    for (mut text, score_text) in score_texts.iter_mut() {
        if let Ok(team) = teams.get(score_text.team) {
            text.0 = team.current_score.to_string();
        }
    }
}