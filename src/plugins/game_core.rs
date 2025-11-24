use leafwing_input_manager::prelude::{ActionState, InputMap};
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::bundles::area::AreaBundle;
use crate::components::*;
use crate::resources::controls::MenuAction;
use crate::systems::*;
use crate::utils::screen::{BALL_RADIUS, HALF_WIDTH, PADDLE_SIZE, PADDLE_WALL_PADDING};

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
            (
                move_paddle,
                check_connection,
                maintain_ball_speed,
                paddle_hit_dynamics,
                update_score_ui,
                detect_button_press
            ))
            .add_systems(Startup, (
                //setup,
                setup_common
            ));
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

    commands.spawn(
        InputMap::default()
            .with(MenuAction::Confirm, KeyCode::Enter)
            .with(MenuAction::Confirm, MouseButton::Left)
            .with(MenuAction::Confirm, KeyCode::Space)
            .with(MenuAction::Confirm, GamepadButton::South)
            .with(MenuAction::Cancel, KeyCode::Escape)
            .with(MenuAction::Cancel, GamepadButton::East)
    );


    spawn_main_menu(&mut commands);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let teams = [team1, team2];
    AreaBundle::spawn(AreaShape::TwoSide, &mut commands, &teams);

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