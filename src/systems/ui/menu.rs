use crate::bundles::area::AreaBundle;
use crate::bundles::widgets::LabelBundle;
use crate::bundles::{BallBundle, DivisionLineBundle};
use crate::components::ui::{Menu, MenuType, OptionSelector};
use crate::events::ui::widgets::ButtonPressed;
use crate::models::game::area::{AreaShape, AreaSide, PlayerInfo, Team};
use crate::models::game::gameplay::GameMode;
use crate::models::game::settings::ScreenMode;
use crate::models::ui::option::UIOption;
use crate::resources::GameConfig;
use crate::systems::handle_scoring;
use crate::systems::widgets::*;
use crate::utils::BALL_RADIUS;
use bevy::prelude::*;


pub fn m_main() -> impl Bundle {
    (
        m_base(MenuType::MainMenu),
        Children::spawn((
            Spawn(LabelBundle::game_title()),
            Spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    padding: UiRect::new(BUTTON_PADDING, BUTTON_PADDING, BUTTON_PADDING, Val::ZERO),
                    ..default()
                },
                Outline::new(Val::Px(5.0), Val::ZERO, Color::linear_rgb(0.5, 0.5, 0.5)),
                BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
                Children::spawn((
                    Spawn((
                        w_menu_button(Color::srgb(0.2, 0.6, 0.9),
                                      "Offline Play"),
                        Observer::new(on_offline)
                    )),
                    Spawn((
                        w_menu_button(Color::srgb(0.6, 0.3, 0.9),
                                      "Online Play"),
                        Observer::new(on_online)
                    )),
                    Spawn((
                        w_menu_button(Color::srgb(0.5, 0.5, 0.5),
                                      "Settings"),
                        Observer::new(on_settings)
                    )),
                    Spawn((
                        w_menu_button(Color::srgb(0.8, 0.2, 0.2),
                                          "Exit"),
                        Observer::new(on_exit)
                    )),
                ))
            ))
        ))
    )
}

pub fn m_offline() -> impl Bundle {
    (
        m_base(MenuType::OfflinePlayMenu),
        Children::spawn((
            Spawn(w_menu_title("Offline Play")),
            Spawn((
                w_menu_section(),
                Children::spawn((
                    Spawn(w_selector(
                        vec![
                            UIOption::new("2 Players", 2),
                            UIOption::new("3 Players", 3),
                            UIOption::new("4 Players", 4)],
                        0,
                        0,
                        "Number of Players"
                    )),
                    Spawn(w_selector(
                        vec![
                            UIOption::new("Classic", GameMode::Classic),
                            UIOption::new("Modern", GameMode::Modern),
                            UIOption::new("Upside Down", GameMode::UpsideDown),
                            UIOption::new("Blackout", GameMode::Blackout),
                            UIOption::new("Twisted", GameMode::Twisted),
                        ],
                        0,
                        1,
                        "Game Mode",
                    )),
                    Spawn(w_selector(
                        vec![
                            UIOption::new("Two Sides", AreaShape::TwoSide(None)),
                            UIOption::new("Triangular", AreaShape::Triangular(None)),
                            UIOption::new("Cuboid", AreaShape::Cuboid(None))
                        ],
                        0,
                        2,
                        "Arena Shape"
                    )),
                    Spawn(w_selector(
                        vec![
                            UIOption::new("5 Points", 5),
                            UIOption::new("10 Points", 10),
                            UIOption::new("15 Points", 15),
                            UIOption::new("20 Points", 20),
                        ],
                        0,
                        3,

                        "Win Score"
                    )),
                ))
            )),
            Spawn((
                Node {
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(Val::Px(30.0)),
                column_gap: Val::Px(20.0),
                ..default()
                },
                Children::spawn((
                    Spawn((
                        w_menu_button(
                        Color::srgb(0.2, 0.7, 0.3),
                        "Start Game"),
                        Observer::new(on_start_offline_game)
                    )),
                    Spawn((
                            w_menu_button(
                            Color::srgb(0.6, 0.6, 0.6),
                            "Back"),
                        Observer::new(on_back_main)
                    ))
                ))
            ))
        ))
    )
}

// Observer callbacks
fn on_quick_match(_press: On<ButtonPressed>) {
    println!("Searching for quick match...");
}

fn on_create_room(_press: On<ButtonPressed>) {
    println!("Creating room...");
}

fn on_join_room(_press: On<ButtonPressed>) {
    println!("Join room menu...");
}

fn on_friends_list(_press: On<ButtonPressed>) {
    println!("Opening friends list...");
}

fn on_offline(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<Menu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_offline());
}

fn on_online(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<Menu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_online());
}

fn on_settings(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<Menu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_settings());
}

fn on_exit(_press: On<ButtonPressed>, mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

fn on_back_main(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    settings_menu: Query<Entity, With<Menu>>,
) {
    let entity = settings_menu.single().expect("Settings Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_main());
}

fn on_start_offline_game(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    offline_menu: Query<Entity, With<Menu>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_config: ResMut<GameConfig>,
    player_count_selector: Query<&OptionSelector>,
    players: Query<Entity, With<crate::components::Player>>,
) {
    // Get player count from selector
    let player_count = if let Ok(selector) = player_count_selector.single() {
        *selector.options[selector.selected].get_value::<i32>().unwrap_or(&2) as usize
    } else {
        2
    };

    // Collect player entities
    let player_entities: Vec<Entity> = players.iter().take(player_count).collect();

    match game_config.area_shape {
        AreaShape::TwoSide(_) => {
            let players_per_team = player_count / 2;
            let team1_players: Vec<PlayerInfo> = player_entities
                .iter()
                .take(players_per_team)
                .enumerate()
                .map(|(i, &entity)| PlayerInfo {
                    name: format!("Player {}", i + 1),
                    entity,
                })
                .collect();

            let team2_players: Vec<PlayerInfo> = player_entities
                .iter()
                .skip(players_per_team)
                .enumerate()
                .map(|(i, &entity)| PlayerInfo {
                    name: format!("Player {}", i + players_per_team + 1),
                    entity,
                })
                .collect();

            game_config.area_shape = AreaShape::TwoSide(Some([
                Team {
                    name: "Left Team".to_string(),
                    current_score: 0,
                    area_side: AreaSide::Left,
                    players: team1_players,
                },
                Team {
                    name: "Right Team".to_string(),
                    current_score: 0,
                    area_side: AreaSide::Right,
                    players: team2_players,
                },
            ]));
        }
        AreaShape::Triangular(_) => {
            let players_per_team = player_count / 3;
            let mut teams = Vec::new();
            let sides = [AreaSide::Left, AreaSide::Top, AreaSide::Right];

            for (team_idx, &side) in sides.iter().enumerate() {
                let team_players: Vec<PlayerInfo> = player_entities
                    .iter()
                    .skip(team_idx * players_per_team)
                    .take(players_per_team)
                    .enumerate()
                    .map(|(i, &entity)| PlayerInfo {
                        name: format!("Player {}", team_idx * players_per_team + i + 1),
                        entity,
                    })
                    .collect();

                teams.push(Team {
                    name: format!("Team {}", team_idx + 1),
                    current_score: 0,
                    area_side: side,
                    players: team_players,
                });
            }

            game_config.area_shape = AreaShape::Triangular(Some([
                teams[0].clone(),
                teams[1].clone(),
                teams[2].clone(),
            ]));
        }
        AreaShape::Cuboid(_) => {
            let players_per_team = player_count / 4;
            let mut teams = Vec::new();
            let sides = [AreaSide::Left, AreaSide::Top, AreaSide::Right, AreaSide::Bottom];

            for (team_idx, &side) in sides.iter().enumerate() {
                let team_players: Vec<crate::models::game::area::PlayerInfo> = player_entities
                    .iter()
                    .skip(team_idx * players_per_team)
                    .take(players_per_team)
                    .enumerate()
                    .map(|(i, &entity)| crate::models::game::area::PlayerInfo {
                        name: format!("Player {}", team_idx * players_per_team + i + 1),
                        entity,
                    })
                    .collect();

                teams.push(Team {
                    name: format!("Team {}", team_idx + 1),
                    current_score: 0,
                    area_side: side,
                    players: team_players,
                });
            }

            game_config.area_shape = AreaShape::Cuboid(Some([
                teams[0].clone(),
                teams[1].clone(),
                teams[2].clone(),
                teams[3].clone(),
            ]));
        }
    }

    // Despawn menu
    let entity = offline_menu.single().expect("Offline menu doesn't exist");
    commands.entity(entity).despawn();

    commands.entity(offline_menu.single().expect("No menu")).despawn();


    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-300.0, 300.0),
        BALL_RADIUS
    )).observe(handle_scoring);

    AreaBundle::spawn(&mut game_config.area_shape, &mut commands, &mut meshes, &mut materials);

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


pub fn m_online() -> impl Bundle {
    (
        m_base(MenuType::OnlinePlayMenu),
        Children::spawn_one((
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
            Text::new("ONLINE PLAY"),
            TextFont {
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 1.0)),
            Children::spawn((
                Spawn((
                    w_menu_section(),
                    Children::spawn((
                        Spawn((
                            w_menu_button(
                                Color::srgb(0.3, 0.6, 0.9),
                                "Quick Match"
                            ),
                            Observer::new(on_quick_match)
                        )),
                        Spawn((
                            w_menu_button(
                                Color::srgb(0.5, 0.4, 0.9),
                                "Create Room",
                            ),
                            Observer::new(on_create_room)
                        )),
                        Spawn((
                            w_menu_button(
                                Color::srgb(0.9, 0.5, 0.3),
                                "Join Room",
                            ),
                            Observer::new(on_join_room)
                        )),
                        Spawn((
                            w_menu_button(
                                Color::srgb(0.4, 0.7, 0.4),
                                "Friends List",
                            ),
                            Observer::new(on_friends_list)
                        )),
                    )),
                )),
                Spawn((
                    w_menu_button(
                        Color::srgb(0.6, 0.6, 0.6),
                        "Back",
                    ),
                    Observer::new(on_back_main)
                ))
            ))
        ))
    )
}

pub fn m_settings() -> impl Bundle {
    (
        m_base(MenuType::SettingsMenu),
        Children::spawn((
            Spawn(w_menu_title("SETTINGS")),
            Spawn((
                w_menu_section(),
                Children::spawn((
                    Spawn((
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                        Text::new("Audio"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.9)),
                    )),
                    Spawn((
                        w_menu_section(),
                        Children::spawn((
                            Spawn(w_slider(0.0, 100.0, 50.0, 0)),
                            Spawn(w_slider(0.0, 100.0, 50.0, 1)),
                            Spawn((
                                Node {
                                    margin: UiRect::vertical(Val::Px(20.0)),
                                    ..default()
                                },
                                Text::new("Graphics"),
                                TextFont {
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.8, 0.9)),
                            )),
                            Spawn(w_selector(
                                vec![
                                    UIOption::new("Handle later", 0),
                                ],
                                0,
                                2,
                                "Resolution".into(),
                            )),
                            Spawn(w_selector(
                                vec![
                                    UIOption::new("Exclusive FullScreen", ScreenMode::ExclusiveFullScreen),
                                    UIOption::new("FullScreen", ScreenMode::BorderlessFullScreen),
                                    UIOption::new("Windowed", ScreenMode::Windowed),
                                ],
                                0,
                                3,
                                "Screen Mode",
                            ))
                        ))
                    ))
                ))
            )),
            Spawn((
                w_menu_button(
                    Color::srgb(0.6, 0.6, 0.6),
                    "Back"
                ),
                Observer::new(on_back_main)
            ))
        ))
    )
}

fn m_base(menu_type: MenuType) -> impl Bundle {
    (
        Menu::new(menu_type),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.1))
    )
}