use crate::bundles::widgets::LabelBundle;
use crate::components::ui::{MainMenu, OfflinePlayMenu, OnlinePlayMenu, OptionSelector, PlayerJoinInMenu, SettingsMenu, SettingsSelector, SourceHandle, UIOptionProvider, UIOptionString};
use crate::components::Player;
use crate::events::widgets::{ButtonPressed, OptionChanged, SliderValueChanged};
use crate::models::game::gameplay::GameMode;
use crate::resources::{GameModeConfig, GameSettings, Monitors, PendingSettings, PlayerAction, Resolution};
use crate::systems::settings::persistence::save_settings;
use crate::systems::widgets::*;
use crate::utils::MODERN_THEME;
use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
use bevy::input_focus::directional_navigation::DirectionalNavigationMap;
use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy::ui_widgets::observe;
use bevy::window::WindowMode;
use leafwing_input_manager::action_state::ActionState;

pub fn spawn_m_main(
    directional_nav_map: &mut DirectionalNavigationMap,
    commands: &mut Commands,
) {

    commands.spawn(m_base(MainMenu)).with_children(| base |{

        base.spawn(LabelBundle::game_title());
        base.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                padding: UiRect::new(BUTTON_PADDING, BUTTON_PADDING, BUTTON_PADDING, Val::ZERO),
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            Outline::new(Val::Px(5.0), Val::ZERO, Color::linear_rgb(0.5, 0.5, 0.5)),
            BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1))
        )).with_children(| cont |{

            let but1 =
            cont.spawn(w_menu_button(Color::srgb(0.2, 0.6, 0.9), "Offline Play"))
                .observe(on_offline).id();

            let but2 =
            cont.spawn(w_menu_button(Color::srgb(0.6, 0.3, 0.9), "Online Play"))
                .observe(on_online).id();

            let but3 =
            cont.spawn(w_menu_button(Color::srgb(0.5, 0.5, 0.5), "Settings"))
                .observe(on_settings).id();

            let but4 =
            cont.spawn(w_menu_button(Color::srgb(0.8, 0.2, 0.2), "Exit"))
                .observe(on_exit).id();

            directional_nav_map.add_looping_edges(
                &[but1,but2,but3,but4],
                CompassOctant::South);
        });
    });
}

#[macro_export]
macro_rules! boxed_vec {
    ($($x:expr),+ $(,)?) => {
        {
            Box::new(vec![$($x),+])
        }
    };
}


const GAMEMODE_OPTIONS: SourceHandle<dyn UIOptionProvider> =
SourceHandle::Static(&[
    GameMode::Classic,
    GameMode::Modern,
    GameMode::UpsideDown,
    GameMode::Blackout,
    GameMode::Twisted,
]);


pub fn m_offline() -> impl Bundle {
    (
        m_base(OfflinePlayMenu),
        children![
            w_menu_title("Offline Play"),
            (
                w_menu_section(),
                children![
                    w_selector(
                        GAMEMODE_OPTIONS,
                        0,
                        "Game Mode",
                    ),
                    w_slider(5.0, 30.0, 5.0)
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
                        observe(on_offline_back_main)
                    )
                ]
            ),
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
    config: Res<GameModeConfig>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_offline());
}

fn on_online(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_online());
}

fn on_settings(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
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

fn on_settings_back_main(
    _: On<ButtonPressed>,
    mut commands: Commands,
    settings_menu: Query<Entity, With<SettingsMenu>>,
    mut nam_map: ResMut<DirectionalNavigationMap>,
    settings: Res<GameSettings>,
) {
    let entity = settings_menu.single().expect("Settings Menu doesn't exist");
    commands.entity(entity).despawn();

    spawn_m_main(nam_map.as_mut(), &mut commands);
    save_settings(&settings);
}

fn on_offline_back_main(
    _: On<ButtonPressed>,
    mut commands: Commands,
    mut map: ResMut<DirectionalNavigationMap>,
    main_menu: Query<Entity, With<OfflinePlayMenu>>,
){
    commands.entity(main_menu.single().expect("No menu")).despawn();
    spawn_m_main(map.as_mut(), &mut commands);
}

fn on_start_offline_game(
    _: On<ButtonPressed>,
    mut commands: Commands,
    menus: Query<Entity, With<OfflinePlayMenu>>,
) {
    let entity = menus.single().expect("No menu found");
    commands.entity(entity).despawn();
    commands.spawn(m_player_join_in(1));
}

fn m_player_join_in(player_num: u8) -> impl Bundle {
    (
        m_base(PlayerJoinInMenu(player_num)),
        children![
            w_menu_title(format!("Player {} Join In", player_num)),
            (
                w_menu_section(),
                children![
                    LabelBundle::button_label("Press any button to join..."),
                ],
            ),
        ],
    )
}


pub fn u_join_in(
    menus: Query<(Entity, &PlayerJoinInMenu)>,
    player_query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_settings: ResMut<GameModeConfig>
) {
    let area = &mut game_settings.area_shape;

    if let Ok((menu, join_in)) = menus.single(){
        let player_num = join_in.0 as usize;

        for (action, player) in player_query {
            if !action.get_just_pressed().is_empty() && !area.contains_player(player.id) {

                let teams_len = area.get_teams().len();

                area.get_teams_mut()[player_num - 1].players.push(player.id);

                commands.entity(menu).despawn();

                if player_num < teams_len {
                    commands.spawn(m_player_join_in(join_in.0 + 1));
                }else{
                    area.spawn(&mut commands, meshes.as_mut(), materials.as_mut());
                }
            }
        }
    }
}



pub fn m_online() -> impl Bundle {
    (
        m_base(OnlinePlayMenu),
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
                observe(on_offline_back_main)
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
    commands.spawn(m_base(SettingsMenu)).with_children(|base| {

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


                    section.spawn(LabelBundle::button_label("Sound Effects"));
                    section.spawn(w_slider(
                        0.0,
                        100.0,
                        settings.sfx_volume
                    )).observe(on_sfx_changed);

                    section.spawn(LabelBundle::button_label("Master volume"));
                    section.spawn(w_slider(
                        0.0,
                        100.0,
                        settings.master_volume
                    )).observe(on_master_changed);

                    let monitor_index = monitors.selected_monitor;
                    let monitor = monitors.get_current_monitor();
                    let mut current_video_mode = VideoModeSelection::Current;

                    let current_window_mode = match settings.video_mode {
                        WindowMode::Windowed => 0,
                        WindowMode::BorderlessFullscreen(..) => 1,
                        WindowMode::Fullscreen(.., window_mode) => {
                            current_video_mode = window_mode;
                            2
                        }
                    };

                    section.spawn(
                        w_selector(
                            SourceHandle::Unique(
                            boxed_vec![
                                WindowMode::Windowed,
                                WindowMode::BorderlessFullscreen(monitor.monitor_selection),
                                WindowMode::Fullscreen(monitor.monitor_selection, current_video_mode),
                            ]),
                            current_window_mode,
                            "Window Mode"
                        ));


                section.spawn(w_selector(
                    SourceHandle::Strong(monitors.monitors.clone()),
                    monitor_index,
                    "Monitor"))
                        .insert(SettingsSelector::Monitor)
                        .observe(on_monitor_changed);

                    section.spawn(w_selector(
                        SourceHandle::Strong(monitor.resolutions.clone()),
                        0,
                        "Resolution"))
                        .insert(SettingsSelector::Resolution)
                        .observe(on_resolution_changed);

                    section.spawn(w_selector(
                        SourceHandle::Strong(monitor.refresh_rates.clone()),
                        0,
                        "Refresh Rate"))
                        .insert(SettingsSelector::RefreshRate)
                        .observe(on_refresh_rate_changed);
                });

            container.spawn(w_button(MODERN_THEME.button, Vec2::new(200.0, 20.0), "Back"))
                    .observe(on_settings_back_main);

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

fn on_settings_apply(change : On<OptionChanged>){
    
}


fn m_base(menu_type: impl Component) -> impl Bundle {
    (
        menu_type,
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

}

fn on_vsync_changed(
    change: On<OptionChanged>,
    selectors: Query<&OptionSelector>,
    mut settings: ResMut<GameSettings>,
    mut windows: Query<&mut Window>,
) {

}

fn on_show_fps_changed(
    change: On<OptionChanged>,
    selectors: Query<&OptionSelector>,
    mut settings: ResMut<GameSettings>,
    mut fps_overlay: ResMut<FpsOverlayConfig>,
) {
    for selector in selectors.iter() {

        let res = selector.current::<Resolution>();
    }
}

fn on_resolution_changed(
    change: On<OptionChanged>,
    selectors: Query<&OptionSelector>,
){
    if let Ok(selector) = selectors.get(change.0){
        if let Some(resolution) = selector.current::<Resolution>(){
            println!("Changed resolution to {}x{}", resolution.x, resolution.y);
        }
    }
}

fn on_refresh_rate_changed(
    change: On<OptionChanged>,
){

}


impl UIOptionString for WindowMode{
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match self { 
            WindowMode::Windowed => "Windowed",
            WindowMode::BorderlessFullscreen(..) => "BorderlessFullscreen",
            WindowMode::Fullscreen(..) => "Fullscreen",
        };
        
        string.push_str(s);
    }
}