use crate::bundles::widgets::LabelBundle;
use crate::components::ui::{Menu, MenuType, OptionSelector, SettingsSelector, SourceHandle, UIOptionString};
use crate::events::widgets::{ButtonPressed, OptionChanged, SliderValueChanged};
use crate::resources::{GameModeConfig, GameSettings, Monitors, PendingSettings, Resolution};
use crate::systems::widgets::*;
use bevy::dev_tools::fps_overlay::FpsOverlayConfig;
use bevy::prelude::*;
use bevy::ui_widgets::observe;
use bevy::window::WindowMode;
use crate::systems::settings::persistence::save_settings;
use crate::utils::MODERN_THEME;

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

#[macro_export]
macro_rules! boxed_vec {
    ($($x:expr),+ $(,)?) => {
        {
            Box::new(vec![$($x),+])
        }
    };
}



pub fn spawn_m_offline(
    settings: &Res<GameModeConfig>,
    commands: &mut Commands,
) {
    commands.spawn(m_base(MenuType::OfflinePlayMenu))
            .with_children(| base |{

                base.spawn(w_title("Select Area Shape", 32.0));

                base.spawn(w_menu_section()).with_children(| section | {

                    section.spawn(w_row_container(30.0)).with_children(| cont | {
                        cont.spawn(w_container(Vec2::new(200.0, 200.0)));
                        cont.spawn(w_container(Vec2::new(200.0, 200.0)));
                    });

                    section.spawn(w_row_container(30.0)).with_children(| cont | {
                        cont.spawn(w_container(Vec2::new(200.0, 200.0)));
                        cont.spawn(w_container(Vec2::new(200.0, 200.0)));
                    });

                });

                base.spawn(w_button(MODERN_THEME.button, Vec2::new(200.0, 50.0), "Back"))
                    .observe(on_offline_back_main);
            });
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
    main_menu: Query<Entity, With<Menu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    spawn_m_offline(&config, &mut commands);
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

fn on_settings_back_main(
    _: On<ButtonPressed>,
    mut commands: Commands,
    settings_menu: Query<Entity, With<Menu>>,
    settings: Res<GameSettings>,
) {
    let entity = settings_menu.single().expect("Settings Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn(m_main());

    save_settings(&settings);
}

fn on_offline_back_main(
    _: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<Menu>>,
){
    commands.entity(main_menu.single().expect("No menu")).despawn();
    commands.spawn(m_main());
}

fn on_start_offline_game(
    _: On<ButtonPressed>,
) {

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
                            ""
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