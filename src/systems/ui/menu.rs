use crate::bundles::area::AreaBundle;
use crate::bundles::widgets::LabelBundle;
use crate::bundles::{BallBundle, DivisionLineBundle};
use crate::components::ui::{Menu, MenuType, OptionSelector, SettingsSelector};
use crate::events::widgets::{ButtonPressed, OptionChanged, SliderValueChanged};
use crate::models::game::area::{AreaShape, AreaSide, PlayerInfo, TeamInfo};
use crate::models::game::gameplay::{GameMode, PlayerNum};
use crate::models::ui::option::UIOption;
use crate::resources::{BitDepth, GameModeConfig, GameSettings, Monitors, PendingSettings, ToUIOptions};
use crate::systems::handle_scoring;
use crate::systems::widgets::*;
use crate::utils::BALL_RADIUS;
use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
use bevy::prelude::*;
use bevy::ui_widgets::observe;
use bevy::window::WindowMode;

pub fn m_main() -> impl Bundle {
    (
        m_base(MenuType::MainMenu),
        children![
            LabelBundle::game_title(),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_wrap: FlexWrap::Wrap,
                    padding: UiRect::new(BUTTON_PADDING, BUTTON_PADDING, BUTTON_PADDING, Val::ZERO),
                    width: Val::Auto,
                    height: Val::Auto,
                    ..default()
                },
                Outline::new(Val::Px(5.0), Val::ZERO, Color::linear_rgb(0.5, 0.5, 0.5)),
                BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
                children![
                    (
                        w_menu_button(Color::srgb(0.2, 0.6, 0.9),
                                      "Offline Play"),
                        observe(on_offline),
                    ),
                    (
                        w_menu_button(Color::srgb(0.6, 0.3, 0.9),
                                      "Online Play"),
                        observe(on_online)
                    ),
                    (
                        w_menu_button(Color::srgb(0.5, 0.5, 0.5),
                                      "Settings"),
                        observe(on_settings)
                    ),
                    (
                        w_menu_button(Color::srgb(0.8, 0.2, 0.2),
                                          "Exit"),
                        observe(on_exit)
                    )
                ]
            )
        ]
    )
}

pub fn m_offline() -> impl Bundle {
    (
        m_base(MenuType::OfflinePlayMenu),
        children![
            w_menu_title("Offline Play"),
            (
                w_menu_section(),
                children![
                    w_selector(
                        vec![
                            PlayerNum(1),
                            PlayerNum(2),
                            PlayerNum(3),
                            PlayerNum(4)
                        ].into(),
                        0,
                        "Number of Players"
                    ),
                    w_selector(
                        vec![
                            GameMode::Classic,
                            GameMode::Modern,
                            GameMode::UpsideDown,
                            GameMode::Blackout,
                            GameMode::Twisted,
                        ].into(),
                        0,
                        "Game Mode",
                    ),
                    w_selector(
                        vec![

                        ],
                        0,
                        "Arena Shape"
                    ),
                    LabelBundle::button_label("Win Score"),
                    w_slider(
                        0.0,
                        100.0,
                        50.0,
                    )
                ],
            ),
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    margin: UiRect::top(Val::Px(30.0)),
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                children![
                    (
                        w_menu_button(
                            Color::srgb(0.2, 0.7, 0.3),
                            "Start Game"),
                        observe(on_start_offline_game)
                    ),
                    (
                        w_menu_button(
                            Color::srgb(0.6, 0.6, 0.6),
                            "Back"),
                        observe(on_back_main)
                    )
                ]
            )
        ],
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
    settings: Res<GameSettings>,
    monitors: Res<Monitors>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    spawn_m_settings(&settings, &monitors, &mut commands);
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
    mut game_config: ResMut<GameModeConfig>,
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
                TeamInfo {
                    name: "Left Team".to_string(),
                    current_score: 0,
                    area_side: AreaSide::Left,
                    players: team1_players,
                },
                TeamInfo {
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

                teams.push(TeamInfo {
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

                teams.push(TeamInfo {
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
        children![
            w_menu_title("Online Play"),
            (
                w_menu_section(),
                children![
                    (
                        w_menu_button(
                            Color::srgb(0.3, 0.6, 0.9),
                            "Quick Match"
                        ),
                        observe(on_quick_match)
                    ),
                    (
                        w_menu_button(
                            Color::srgb(0.5, 0.4, 0.9),
                            "Create Room",
                        ),
                        observe(on_create_room)
                    ),
                    (
                        w_menu_button(
                            Color::srgb(0.9, 0.5, 0.3),
                            "Join Room",
                        ),
                        observe(on_join_room)
                    ),
                    (
                        w_menu_button(
                            Color::srgb(0.4, 0.7, 0.4),
                            "Friends List",
                        ),
                        observe(on_friends_list)
                    ),
                ],
            ),
            (
                w_menu_button(
                    Color::srgb(0.6, 0.6, 0.6),
                    "Back",
                ),
                observe(on_back_main)
            )
        ],
    )
}

pub fn spawn_m_settings(
    settings: &Res<GameSettings>,
    monitors: &Res<Monitors>,
    commands: &mut Commands,
) {
    commands.insert_resource(PendingSettings::from(settings));
    commands.spawn(m_base(MenuType::SettingsMenu)).with_children(|base| {

        base.spawn(w_menu_title("Settings"));

        base.spawn(Node {
                flex_direction: FlexDirection::Column,
                width: Val::Px(1000.0),
                max_height: Val::Px(600.0),
                overflow: Overflow::clip_y(),
                ..default()
            }).with_children(|container| {
            container.spawn(w_menu_section())
                .with_children(| section |{

                    let options = monitors.monitors.to_ui_options();
                    let monitor_index = monitors.selected_monitor.unwrap_or_default();
                    let monitor = monitors.get_current_monitor_or_first().expect("No Monitors");
                    let refresh_rates = monitor.refresh_rates
                        .iter()
                        .map(|rate| UIOption::new(format!("{}Hz", *rate / 1000), *rate))
                        .collect();


                section.spawn(w_selector(
                        options,
                        monitor_index,
                    "Monitor"))
                        .insert(SettingsSelector::Monitor)
                        .observe(on_monitor_changed);

                    section.spawn(w_selector(
                        monitor.resolutions.to_ui_options(),
                        0,
                        "Resolution"))
                        .insert(SettingsSelector::Resolution)
                        .observe(on_monitor_changed);

                    section.spawn(w_selector(
                        refresh_rates,
                        0,
                        "Refresh Rate"))
                        .insert(SettingsSelector::RefreshRate)
                        .observe(on_monitor_changed);
                });
            container.spawn(LabelBundle::button_label(""));
        });

    });
}

fn on_sfx_changed(change: On<SliderValueChanged>, mut settings: ResMut<GameSettings>){
    settings.sfx_volume = change.value;
    println!("Changed SFX volume to {}", change.value);
}

fn on_master_changed(change: On<SliderValueChanged>, mut settings: ResMut<GameSettings>){
    settings.master_volume = change.value;
    println!("Changed MASTER volume to {}", change.value);
}

fn on_screen_mode_changed(change : On<OptionChanged>){

}

fn on_settings_apply(){

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

fn on_monitor_changed(
    change: On<OptionChanged>,
    selectors: Query<&OptionSelector>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(selector) = selectors.get(change.event_target()) {
        if let Some(monitor_idx) = selector.options[selector.selected].get_value::<usize>() {
            if let Ok(mut window) = windows.single_mut() {
                window.mode = WindowMode::Fullscreen(
                    MonitorSelection::Index(*monitor_idx),
                    VideoModeSelection::Current
                );
                println!("Changed to monitor {}", monitor_idx);
            }
        }
    }
}

fn on_vsync_changed(
    change: On<OptionChanged>,
    selectors: Query<&OptionSelector>,
    mut settings: ResMut<GameSettings>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(selector) = selectors.get(change.event_target()) {
        if let Some(enabled) = selector.options[selector.selected].get_value::<bool>() {
            settings.vsync = *enabled;

            if let Ok(mut window) = windows.single_mut() {
                window.present_mode = if *enabled {
                    bevy::window::PresentMode::AutoVsync
                } else {
                    bevy::window::PresentMode::AutoNoVsync
                };
            }

            println!("VSync {}", if *enabled { "enabled" } else { "disabled" });
        }
    }
}

fn on_show_fps_changed(
    change: On<OptionChanged>,
    selectors: Query<&OptionSelector>,
    mut settings: ResMut<GameSettings>,
    mut fps_overlay: ResMut<FpsOverlayConfig>,
) {
    if let Ok(selector) = selectors.get(change.event_target()) {
        if let Some(show) = selector.options[selector.selected].get_value::<bool>() {

            settings.show_fps = *show;
            fps_overlay.enabled = *show;

            println!("FPS counter {}", if *show { "shown" } else { "hidden" });
        }
    }
}