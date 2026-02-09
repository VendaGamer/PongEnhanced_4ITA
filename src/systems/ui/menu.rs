use avian2d::parry::na::DimAdd;
use crate::bundles::area::AreaBundle;
use crate::bundles::widgets::LabelBundle;
use crate::components::ui::{MainMenu, MonitorSelector, OfflinePlayMenu, OnlineCreateMenu, OnlinePlayMenu, PlayerJoinInMenu, RefreshRateSelector, ResolutionSelector, Selector, SettingsMenu, SourceHandle, UIOptionProvider, UIOptionString, VSyncSelector, WindowModeSelector};
use crate::components::Player;
use crate::events::widgets::{ButtonPressed, OptionChanged, SliderValueChanged, TextInputSubmitted};
use crate::models::game::gameplay::GameMode;
use crate::models::ui::option::{VSYNC_OPTIONS, VSYNC_OPTIONS_RAW};
use crate::resources::{GameModeConfig, GameSettings, MonitorInfo, Monitors, OnlineGameConfig, PendingSettings, PlayerAction, RefreshRate, Resolution};
use crate::systems::settings::persistence::save_settings;
use crate::systems::widgets::*;
use crate::utils::MODERN_THEME;
use bevy::ecs::query::Spawned;
use bevy::input_focus::directional_navigation::DirectionalNavigationMap;
use bevy::math::CompassOctant;
use bevy::prelude::*;
use bevy::reflect::Array;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::window::{PresentMode, PrimaryWindow, VideoMode, WindowMode};
use bevy_simple_text_input::TextInputSubmitMessage;
use leafwing_input_manager::action_state::ActionState;
use crate::networking::server::start_server;

pub const GAMEMODE_OPTIONS: SourceHandle<dyn UIOptionProvider> = SourceHandle::Static(&GAMEMODE_OPTIONS_RAW);

pub const GAMEMODE_OPTIONS_RAW: [GameMode; 5] = [
    GameMode::Classic,
    GameMode::Modern,
    GameMode::UpsideDown,
    GameMode::Blackout,
    GameMode::Twisted,
];

#[inline]
fn index_of_game_mode(game_mode: &GameMode) -> usize {
    GAMEMODE_OPTIONS_RAW.iter().position(|r| r == game_mode).unwrap_or(0)
}

#[inline]
fn current_video_mode(
    settings: &GameSettings,
    monitor: &MonitorInfo,
) -> VideoMode {
    match settings.window_mode {
        WindowMode::Fullscreen(.., selection) => {
            match selection {
                VideoModeSelection::Specific(mode) => mode,
                _ => monitor.native_mode
            }
        }
        _ => monitor.native_mode
    }
}

#[inline]
fn current_resolution_index(
    resolutions: &Vec<Resolution>,
    video_mode: &VideoMode
) -> usize {
    resolutions.iter().position(| res | res.0 == video_mode.physical_size).unwrap_or(resolutions.len() - 1)
}

#[inline]
fn current_refresh_rate_index(
    resolutions: &Vec<RefreshRate>,
    video_mode: &VideoMode
) -> usize {
    resolutions.iter().position(|ref_rate| ref_rate.0 == video_mode.refresh_rate_millihertz).unwrap_or(resolutions.len() - 1)
}

pub fn spawn_m_main<'a>(commands: &'a mut Commands, nav_map: &'a mut DirectionalNavigationMap) -> EntityCommands<'a> {

    let mut base = spawn_m_base(commands, nav_map, MainMenu);

    base.with_children(|parent| {
        parent.spawn(LabelBundle::game_title());
        parent.spawn((
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
        )).with_children(|cont| {
            let but1 = cont
                .spawn(w_menu_button(Color::srgb(0.2, 0.6, 0.9), "Offline Play"))
                .observe(on_offline)
                .id();

            let but2 = cont
                .spawn(w_menu_button(Color::srgb(0.6, 0.3, 0.9), "Online Play"))
                .observe(on_online)
                .id();

            let but3 = cont
                .spawn(w_menu_button(Color::srgb(0.5, 0.5, 0.5), "Settings"))
                .observe(on_settings)
                .id();

            let but4 = cont
                .spawn(w_menu_button(Color::srgb(0.8, 0.2, 0.2), "Exit"))
                .observe(on_exit)
                .id();

            nav_map.add_looping_edges(&[but1, but2, but3, but4], CompassOctant::South);
        });
    });

    return base;

    fn on_offline(
        _press: On<ButtonPressed>,
        config: Res<GameModeConfig>,
        mut commands: Commands,
        mut nav_map: ResMut<DirectionalNavigationMap>,
        main_menu: Query<Entity, With<MainMenu>>,
    ) {
        let entity = main_menu.single().expect("Main Menu doesn't exist");
        commands.entity(entity).despawn();
        spawn_m_offline(&mut commands, &mut nav_map, &config);
    }

    fn on_online(
        _press: On<ButtonPressed>,
        mut commands: Commands,
        mut nav_map: ResMut<DirectionalNavigationMap>,
        main_menu: Query<Entity, With<MainMenu>>,
    ) {
        let entity = main_menu.single().expect("Main Menu doesn't exist");
        commands.entity(entity).despawn();
        spawn_m_online(&mut commands, &mut nav_map);
    }

    fn on_settings(
        _press: On<ButtonPressed>,
        mut commands: Commands,
        mut nav_map: ResMut<DirectionalNavigationMap>,
        main_menu: Query<Entity, With<MainMenu>>,
        settings: Res<GameSettings>,
        monitors: Res<Monitors>,
    ) {
        let entity = main_menu.single().expect("Main Menu doesn't exist");
        commands.entity(entity).despawn();
        spawn_m_settings(&settings, &monitors, &mut commands, &mut nav_map);
    }

    fn on_exit(_press: On<ButtonPressed>, mut exit: MessageWriter<AppExit>) {
        exit.write(AppExit::Success);
    }
}

#[macro_export]
macro_rules! boxed_vec {
    ($($x:expr),+ $(,)?) => {
        {
            Box::new(vec![$($x),+])
        }
    };
}

pub fn spawn_m_offline<'a>(
    commands: &'a mut Commands,
    nav_map: &'a mut DirectionalNavigationMap,
    config: &'a Res<GameModeConfig>
) -> EntityCommands<'a> {

    let mut base = spawn_m_base(commands, nav_map, OfflinePlayMenu);
    let mut entities: Vec<Entity> = Vec::new();

    base.with_children(|parent| {

        parent.spawn(w_menu_title("Offline Play"));

        parent.spawn(w_menu_section()).with_children(|parent| {

            let mut g_sel = parent.spawn_selector(
                GAMEMODE_OPTIONS,
                index_of_game_mode(&config.game_mode),
                "GameMode",
            );

            g_sel.root.observe(on_game_mode_changed);

            entities.push(g_sel.bar);
        });

        parent.spawn(w_row_container(Val::Px(20.0))).with_children(|parent| {

            entities.push(parent.spawn(
                w_menu_button(Color::srgb(0.2, 0.7, 0.3), "Start Game")
            ).observe(on_start)
             .id());

            entities.push(parent.spawn(
                w_menu_button(Color::srgb(0.6, 0.6, 0.6), "Back")
            ).observe(on_back)
             .id());

        });
    });

    nav_map.add_looping_edges(&entities[..=1], CompassOctant::South);
    nav_map.add_looping_edges(&[entities[0], entities[2]], CompassOctant::South);
    nav_map.add_looping_edges(&entities[1..=2], CompassOctant::East);

    return base;

    fn on_start(
        _: On<ButtonPressed>,
        mut commands: Commands,
        mut nav_map: ResMut<DirectionalNavigationMap>,
        menu: Single<Entity, With<OfflinePlayMenu>>,
    ) {
        commands.entity(*menu).despawn();
        spawn_m_player_join_in(&mut commands, &mut nav_map, 1);
    }
    fn on_back(
        _: On<ButtonPressed>,
        mut commands: Commands,
        mut map: ResMut<DirectionalNavigationMap>,
        menu: Single<Entity, With<OfflinePlayMenu>>,
    ) {
        commands.entity(*menu).despawn();
        spawn_m_main(&mut commands, &mut map);
    }
    fn on_game_mode_changed(
        change: On<OptionChanged>,
        selectors: Query<(Entity, &Selector)>,
        mut config: ResMut<GameModeConfig>,
    ) {
        for (entity, selector) in selectors.iter() {
            if change.entity == entity {
                if let Some(change) = selector.current::<GameMode>() {
                    config.game_mode = *change;

                    println!("Game mode changed to {change:?}");
                }

                break;
            }
        }
    }
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

    spawn_m_main(&mut commands, &mut nam_map);
    save_settings(&settings);
}

fn spawn_m_player_join_in<'a>(commands: &'a mut Commands, nav_map: &'a mut DirectionalNavigationMap, player_num: u8) -> EntityCommands<'a> {

    let mut base = spawn_m_base(commands, nav_map, PlayerJoinInMenu(player_num));

    base.with_children(| parent |{

        parent.spawn(w_menu_title(format!("Player {} Join In", player_num)));
        parent.spawn((
            w_menu_section(),
        )).with_child(LabelBundle::button_label("Press any button to join..."));
    });

    base
}

pub fn spawn_m_online<'a>(
    commands: &'a mut Commands,
    nav_map: &'a mut DirectionalNavigationMap,
) -> EntityCommands<'a> {

    let mut entities: Vec<Entity> = Vec::new();

    let mut base = spawn_m_base(commands, nav_map, OnlinePlayMenu);

    base.with_children(|parent| {

        parent.spawn(w_menu_title("Online Play"));
        parent.spawn(w_menu_section()).with_children(|parent| {

            entities.push(parent.spawn(
                w_menu_button(Color::srgb(0.5, 0.4, 0.9), "Create Room")
            ).observe(on_create_room)
             .id());

            entities.push(parent.spawn(
                w_menu_button(Color::srgb(0.9, 0.5, 0.3), "Join Room")
            ).observe(on_join_room)
             .id());

        });

        entities.push(parent.spawn(
            w_menu_button(Color::srgb(0.6, 0.6, 0.6), "Back")
        ).observe(on_back)
         .id());

    });

    nav_map.add_looping_edges(&entities, CompassOctant::South);

    return base;


    fn on_quick_match(_press: On<ButtonPressed>) {
        println!("Searching for quick match...");
    }

    fn on_create_room(
        _press: On<ButtonPressed>,
        menu: Single<Entity, With<OnlinePlayMenu>>,
        mut commands: Commands,
        mut nav_map: ResMut<DirectionalNavigationMap>,
    ) {
        commands.entity(*menu).despawn();
        spawn_m_online_create_name(&mut commands, &mut nav_map);
    }

    fn on_join_room(_press: On<ButtonPressed>) {
        println!("Join room menu...");
    }

    fn on_friends_list(_press: On<ButtonPressed>) {
        println!("Opening friends list...");
    }

    fn on_back(
        _: On<ButtonPressed>,
        mut commands: Commands,
        mut map: ResMut<DirectionalNavigationMap>,
        main_menu: Single<Entity, With<OnlinePlayMenu>>,
    ) {
        commands.entity(*main_menu).despawn();
        spawn_m_main(&mut commands, &mut map);
    }

}


fn index_for_window_mode(window_mode: &WindowMode) -> usize {
    match window_mode {
        WindowMode::Windowed => 0,
        WindowMode::BorderlessFullscreen(..) => 1,
        WindowMode::Fullscreen(..) => 2,
    }
}

pub fn spawn_m_settings(
    settings: &GameSettings,
    monitors: &Monitors,
    commands: &mut Commands,
    nav_map: &mut DirectionalNavigationMap,
) {
    let cur_window_mode = index_for_window_mode(&settings.window_mode);
    let mut entities: Vec<Entity> = Vec::with_capacity(9);
    commands.insert_resource(PendingSettings::from(settings));


    spawn_m_base(commands, nav_map, SettingsMenu).with_children(|base| {
        base.spawn(w_menu_title("Settings"));

        base.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(30.0)),
                margin: UiRect::all(Val::Px(10.0)),
                width: Val::Px(800.0),
                border: PIXEL_BORDER,
                ..default()
            },
            BackgroundColor(MODERN_THEME.section_bg),
            BorderColor::all(MODERN_THEME.border_dark),
            BorderRadius::ZERO,
        )).with_children(| section | {

            {
                section.spawn(LabelBundle::button_label("Sound Effects"));
                let mut sfx = section.spawn_slider(0.0, 100.0, settings.sfx_volume);

                sfx.root.observe(on_sfx_changed);
                entities.push(sfx.thumb);
            }

            {
                section.spawn(LabelBundle::button_label("Master volume"));
                let mut mas = section.spawn_slider(0.0, 100.0, settings.master_volume);

                mas.root.observe(on_master_changed);
                entities.push(mas.thumb);
            }

            {
                let monitor_index = monitors.selected_monitor;
                let monitor = monitors.get_current_monitor();
                let video_mode = current_video_mode(settings, monitor);

                {
                    let mut w_sel = section.spawn_selector(
                        SourceHandle::Unique(boxed_vec![
                        WindowMode::Windowed,
                        WindowMode::BorderlessFullscreen(monitor.monitor_selection),
                        WindowMode::Fullscreen(
                            monitor.monitor_selection,
                            VideoModeSelection::Specific(video_mode)
                        )
                    ]),
                        cur_window_mode,
                        "Window Mode",
                    );

                    w_sel.root.insert(WindowModeSelector)
                        .observe(on_window_mode_changed);

                    entities.push(w_sel.bar);
                }

                {
                    let mut m_sel = section.spawn_selector(
                        SourceHandle::Strong(monitors.monitors.clone()),
                        monitor_index,
                        "Monitor",
                    );

                    entities.push(m_sel.bar);

                    m_sel.root.insert(MonitorSelector)
                              .observe(on_monitor_changed);
                }

                {
                    let mut res_sel = section.spawn_selector(
                        SourceHandle::Strong(monitor.resolutions.clone()),
                        current_resolution_index(&monitor.resolutions, &video_mode),
                        "Resolution",
                    );

                    entities.push(res_sel.bar);

                    res_sel.root.insert(ResolutionSelector)
                                .observe(on_resolution_changed);
                }

                {
                    let mut ref_sel = section.spawn_selector(
                        SourceHandle::Strong(monitor.refresh_rates.clone()),
                        current_refresh_rate_index(&monitor.refresh_rates, &video_mode),
                        "Refresh Rate",
                    );

                    entities.push(ref_sel.bar);

                    ref_sel.root.insert(RefreshRateSelector)
                                .observe(on_refresh_rate_changed);
                }
            }

            {

                let index = if matches!(settings.vsync, PresentMode::AutoNoVsync) {
                    0
                } else {
                    1
                };

                let mut v_sel = section.spawn_selector(
                    VSYNC_OPTIONS,
                    index,
                    "VSync");

                v_sel.root.insert(VSyncSelector).observe(on_vsync_changed);

                entities.push(v_sel.bar);
            }
        });

        base.spawn(w_row_container(Val::Px(10.0)))
            .with_children(|container| {
                const SIZE: Val2 = Val2::new(Val::Px(300.0), Val::Px(50.0));

                entities.push(
                    container
                        .spawn(w_button(MODERN_THEME.button, "Back", SIZE))
                        .observe(on_settings_back_main)
                        .id(),
                );

                entities.push(
                    container
                        .spawn(w_button(MODERN_THEME.button, "Apply", SIZE))
                        .observe(on_settings_apply)
                        .id(),
                );
            });
    });

    nav_map.add_looping_edges(&entities[..=7], CompassOctant::South);
    nav_map.add_looping_edges(
        &[
            entities[0],
            entities[1],
            entities[2],
            entities[3],
            entities[4],
            entities[5],
            entities[6],
            entities[8],
        ],
        CompassOctant::South,
    );

    nav_map.add_looping_edges(&entities[7..=8], CompassOctant::East);


    fn on_sfx_changed(change: On<SliderValueChanged>, mut settings: ResMut<GameSettings>) {
        settings.sfx_volume = change.value;
        println!("Changed SFX volume to {}", change.value);
    }

    fn on_master_changed(change: On<SliderValueChanged>, mut settings: ResMut<GameSettings>) {
        settings.master_volume = change.value;
        println!("Changed MASTER volume to {}", change.value);
    }


    fn on_window_mode_changed(
        _: On<OptionChanged>,
        mod_sel: Single<&Selector, With<WindowModeSelector>>,
        mut selectors: ParamSet<(
            Single<(&mut Node, &Selector), With<MonitorSelector>>,
            Single<(&mut Node, &Selector), With<ResolutionSelector>>,
            Single<(&mut Node, &Selector), With<RefreshRateSelector>>,
        )>,
        mut settings: ResMut<PendingSettings>,
    ) {
        let current = mod_sel.current::<WindowMode>().unwrap();

        change_selector_visibility(current, &mut selectors);
        settings.window_mode = *current;

        match current {
            WindowMode::Fullscreen(..) =>{
                settings.window_resolution = None;
            },
            WindowMode::BorderlessFullscreen(..) => {
                settings.window_resolution = None;
            },
            WindowMode::Windowed => {
                let current_res = selectors.p1().1.current::<Resolution>().unwrap();

                settings.window_resolution = Some(current_res.0);
            },
        }

    }

    fn on_monitor_changed(
        _: On<OptionChanged>,
        mut selectors: ParamSet<(
            Single<&mut Selector, With<MonitorSelector>>,
            Single<&mut Selector, With<ResolutionSelector>>,
            Single<&mut Selector, With<RefreshRateSelector>>,
        )>,
        mut settings: ResMut<PendingSettings>,
        mut monitors: ResMut<Monitors>,
    ) {
        let last_index = monitors.selected_monitor;

        {
            let mut mon_sel = selectors.p0();

            match settings.window_mode {
                WindowMode::Fullscreen(ref mut monitor, ..) => {
                    let current_monitor = mon_sel.current::<MonitorInfo>().unwrap();

                    *monitor = current_monitor.monitor_selection;
                    monitors.selected_monitor = mon_sel.selected;
                }
                WindowMode::BorderlessFullscreen(ref mut monitor) => {
                    let current_monitor = mon_sel.current::<MonitorInfo>().unwrap();

                    *monitor = current_monitor.monitor_selection;
                    monitors.selected_monitor = mon_sel.selected;
                }
                WindowMode::Windowed => {
                    monitors.selected_monitor = 0;
                }
            };

            if last_index == monitors.selected_monitor {
                return;
            }

            mon_sel.selected = monitors.selected_monitor;
        }

        let current_monitor = monitors.get_current_monitor();

        {
            let mut res_sel = selectors.p1();
            let resolutions = &current_monitor.resolutions;
            let res_value = res_sel.current::<Resolution>().unwrap().0;

            let index = resolutions
                .iter()
                .position(|r| r.0 == res_value)
                .unwrap_or(resolutions.len() - 1);

            res_sel.set(
                SourceHandle::Strong(resolutions.clone()),
                index,
            );
        }

        {
            let mut ref_sel = selectors.p2();
            let ref_rates = &current_monitor.refresh_rates;
            let ref_value = ref_sel.current::<RefreshRate>().unwrap().0;

            let index = ref_rates
                .iter()
                .position(|r| r.0 == ref_value)
                .unwrap_or(ref_rates.len() - 1);

            ref_sel.set(
                SourceHandle::Strong(ref_rates.clone()),
                index
            );
        }
    }

    fn on_vsync_changed(change: On<OptionChanged>, mut settings: ResMut<PendingSettings>) {
        settings.vsync = VSYNC_OPTIONS_RAW[change.selected_index];
    }

    fn on_resolution_changed(
        _: On<OptionChanged>,
        selector: Single<&Selector, With<ResolutionSelector>>,
        mut settings: ResMut<PendingSettings>,
    ) {
        if let Some(res) = selector.current::<Resolution>() {

            if matches!(settings.window_mode, WindowMode::Windowed){
                settings.window_resolution = Some(res.0);
                return;
            }

            if let WindowMode::Fullscreen(.., selection) = &mut settings.window_mode {
                if let VideoModeSelection::Specific(mode) = selection {
                    mode.physical_size = res.0;
                }
            }

            settings.window_resolution = None;
        }
    }

    fn on_refresh_rate_changed(
        _: On<OptionChanged>,
        selectors: Query<&Selector, With<RefreshRateSelector>>,
        mut settings: ResMut<PendingSettings>,
    ) {
        let selector = selectors.single().expect("No resolution selector found");

        if let Some(res) = selector.current::<RefreshRate>() {
            if let WindowMode::Fullscreen(.., selection) = &mut settings.window_mode {
                if let VideoModeSelection::Specific(mode) = selection {
                    mode.refresh_rate_millihertz = res.0;
                }
            }
        }
    }


    fn on_settings_apply(
        _: On<ButtonPressed>,
        pending: Res<PendingSettings>,
        mut settings: ResMut<GameSettings>,
        mut window: Single<&mut Window, With<PrimaryWindow>>,
    ) {
        settings.vsync = pending.vsync;
        settings.window_mode = pending.window_mode;
        settings.window_resolution = pending.window_resolution;
        
        window.present_mode = settings.vsync;
        window.mode = settings.window_mode;

        if let Some(res) = settings.window_resolution {
            if matches!(settings.window_mode, WindowMode::Windowed) {
                window.resolution.set_physical_resolution(res.x, res.y);
            }
        }
    }
}


fn spawn_m_online_create_name<'a>(
    commands: &'a mut Commands,
    nav_map: &mut DirectionalNavigationMap) -> EntityCommands<'a> {
    
    let mut base = spawn_m_base(commands, nav_map, OnlineCreateMenu);

    base.with_children(|parent| {
        parent.spawn_input("Server Name: ")
              .input.observe(on_submit);
    });

    return base;
    
    fn on_submit(
        submit: On<TextInputSubmitted>,
        menu: Single<Entity, With<OnlineCreateMenu>>,
        mut commands: Commands,
        mut nav_map: ResMut<DirectionalNavigationMap>,
        mut config: ResMut<OnlineGameConfig>
    ){
        config.server_name.clear();
        config.server_name.push_str(&submit.value);
        
        commands.entity(*menu).despawn();
        spawn_m_online_create_pass(&mut commands, &mut nav_map);
    }
}

fn spawn_m_online_create_pass<'a>(
    commands: &'a mut Commands,
    nav_map: &mut DirectionalNavigationMap) -> EntityCommands<'a> {

    let mut base = spawn_m_base(commands, nav_map, OnlineCreateMenu);

    base.with_children(|parent| {
        parent.spawn_input("Password: ")
            .input.observe(on_submit);
    });

    return base;

    fn on_submit(
        submit: On<TextInputSubmitted>,
        menu: Single<Entity, With<OnlineCreateMenu>>,
        mut commands: Commands,
        mut config: ResMut<OnlineGameConfig>
    ){
        config.pass = Some(submit.value.clone());

        commands.entity(*menu).despawn();
    }
}



fn spawn_m_base<'a>(commands: &'a mut Commands, nav_map: &mut DirectionalNavigationMap, menu_type: impl Component) -> EntityCommands<'a> {

    nav_map.clear();

    commands.spawn((
        menu_type,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
    ))
}

pub fn u_settings_visibility(
    _ : Single<(), (With<SettingsMenu>, Spawned)>,
    mut selectors: ParamSet<(
        Single<(&mut Node, &Selector), With<MonitorSelector>>,
        Single<(&mut Node, &Selector), With<ResolutionSelector>>,
        Single<(&mut Node, &Selector), With<RefreshRateSelector>>,
    )>,
    settings: Res<PendingSettings>,
) {
    change_selector_visibility(&settings.window_mode, &mut selectors);
}

fn change_selector_visibility(
    window_mode: &WindowMode,
    selectors: &mut ParamSet<(
        Single<(&mut Node, &Selector), With<MonitorSelector>>,
        Single<(&mut Node, &Selector), With<ResolutionSelector>>,
        Single<(&mut Node, &Selector), With<RefreshRateSelector>>,
    )>,
) {
    match window_mode {
        WindowMode::Windowed => {
            selectors.p0().0.display = Display::None;
            selectors.p1().0.display = Display::Flex;
            selectors.p2().0.display = Display::None;
        }
        WindowMode::Fullscreen(..) => {
            selectors.p0().0.display = Display::Flex;
            selectors.p1().0.display = Display::Flex;
            selectors.p2().0.display = Display::Flex;
        }
        WindowMode::BorderlessFullscreen(..) => {
            selectors.p0().0.display = Display::Flex;
            selectors.p1().0.display = Display::None;
            selectors.p2().0.display = Display::None;
        }
    }
}

pub fn u_join_in(
    menus: Single<(Entity, &PlayerJoinInMenu)>,
    player_query: Query<(&ActionState<PlayerAction>, &Player)>,
    mut commands: Commands,
    mut nav_map: ResMut<DirectionalNavigationMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_settings: ResMut<GameModeConfig>,
) {
    let player_num = menus.1.0 as usize;

    for (action, player) in player_query {
        let area = &mut game_settings.area_shape;
        if !action.get_just_pressed().is_empty() && !area.contains_player(player.id) {
            let teams_len = area.get_teams().len();

            area.get_teams_mut()[player_num - 1].players.push(player.id);

            commands.entity(menus.0).despawn();

            if player_num < teams_len {
                spawn_m_player_join_in(&mut commands, &mut nav_map, (player_num + 1) as u8);
            } else {
                AreaBundle::spawn(
                    &game_settings,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
            }
        }
    }
}

impl UIOptionString for WindowMode {
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match self {
            WindowMode::Windowed => "Windowed",
            WindowMode::BorderlessFullscreen(..) => "Borderless Fullscreen",
            WindowMode::Fullscreen(..) => "Fullscreen",
        };

        string.push_str(s);
    }
}