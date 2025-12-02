use bevy::prelude::*;
use crate::bundles::widgets::LabelBundle;
use crate::components::{AreaShape, GameMode};
use crate::components::ui::{Menu, MenuType};
use crate::components::ui::navigation::UINavSlot;
use crate::models::game::fullscreen::ScreenMode;
use crate::models::ui::option::UIOption;
use crate::systems::ButtonPressed;
use crate::systems::widgets::*;

pub trait MenuSpawnCommandsExt {
    fn spawn_main_menu(&mut self) -> EntityCommands<'_>;
    fn spawn_offline_menu(&mut self) -> EntityCommands<'_>;
    fn spawn_online_menu(&mut self) -> EntityCommands<'_>;
    fn spawn_settings_menu(&mut self) -> EntityCommands<'_>;
    fn spawn_menu_base(&mut self, menu: MenuType) -> EntityCommands<'_>;
}

impl<'w, 's> MenuSpawnCommandsExt for Commands<'w, 's> {
    fn spawn_main_menu(&mut self) -> EntityCommands<'_> {

        let mut main_menu = self.spawn_menu_base(MenuType::MainMenu);

        main_menu.with_children(|parent| {
            parent.spawn(
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::new(BUTTON_PADDING, BUTTON_PADDING, BUTTON_PADDING, Val::ZERO),
                        ..default()
                    },
                    Outline::new(Val::Px(5.0), Val::ZERO, Color::linear_rgb(0.5, 0.5, 0.5)),
                    BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
                )
            );

            parent.spawn(LabelBundle::game_title())
                .with_children(|parent|{

                    parent.append_menu_button(
                        Color::srgb(0.2, 0.6, 0.9),
                        "Offline Play",
                        UINavSlot::row(0))
                        .observe(on_offline);

                    parent.append_menu_button(
                        Color::srgb(0.6, 0.3, 0.9),
                        "Online Play",
                        UINavSlot::row(1)
                    ).observe(on_online);

                    parent.append_menu_button(
                        Color::srgb(0.5, 0.5, 0.5),
                        "Settings", UINavSlot::row(2)
                    ).observe(on_settings);

                    parent.append_menu_button(
                        Color::srgb(0.8, 0.2, 0.2),
                        "Exit",
                        UINavSlot::row(3)
                    ).observe(on_exit);

                });
        });

        main_menu
    }

    fn spawn_offline_menu(&mut self) -> EntityCommands<'_> {
        let mut offline_menu =self.spawn_menu_base(MenuType::OfflinePlayMenu);

        offline_menu.with_children(|parent| {
            // Title
            parent.spawn((

            ));

            // Game options container
            parent.append_menu_section()
                .with_children(|section| {
                    section.append_menu_title("Options");
                    // Number of Players
                    section.append_selector(
                        vec![
                            UIOption::int("2 Players", 2),
                            UIOption::int("3 Players", 3),
                            UIOption::int("4 Players", 4)],
                        0,
                            UINavSlot::new(0, 0),
                        "Number of Players"
                    );

                    // Game Mode
                    section.append_selector(
                        vec![
                            UIOption::game_mode("Classic", GameMode::Classic),
                            UIOption::game_mode("Modern", GameMode::Modern),
                            UIOption::game_mode("Upside Down", GameMode::UpsideDown),
                            UIOption::game_mode("Blackout", GameMode::Blackout),
                            UIOption::game_mode("Twisted", GameMode::Twisted),
                        ],
                        0,
                        UINavSlot::new(1, 0),
                        "Game Mode",
                    );

                    // Arena Shape
                    section.append_selector(
                        vec![
                            UIOption::area("Two Sides", AreaShape::TwoSide),
                            UIOption::area("Triangular", AreaShape::Triangular),
                            UIOption::area("Cuboid", AreaShape::Cuboid)
                        ],
                        0,
                        UINavSlot::new(2, 0),
                        "Arena Shape"
                    );


                    // Win Score
                    section.append_selector(
                        vec![
                            UIOption::int("5 Points", 5),
                            UIOption::int("10 Points", 10),
                            UIOption::int("15 Points", 15),
                            UIOption::int("20 Points", 20),
                        ],
                        0,
                        UINavSlot::new(3, 0),
                        "Win Score"
                    );

                });

            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(Val::Px(30.0)),
                column_gap: Val::Px(20.0),
                ..default()
            }).with_children(|buttons| {
                buttons.append_menu_button(
                    Color::srgb(0.2, 0.7, 0.3),
                    "Start Game",
                    UINavSlot::new(4, 0),
                ).observe(on_start_offline_game);

                buttons.append_menu_button(
                    Color::srgb(0.6, 0.6, 0.6),
                    "Back",
                    UINavSlot::new(4, 1),
                ).observe(on_menu_to_main);
            });
        });

        offline_menu
    }

    fn spawn_online_menu(&mut self) -> EntityCommands<'_> {
        let mut online_menu = self.spawn_menu_base(MenuType::OnlinePlayMenu);

        online_menu.with_children(|parent| {
            // Title
            parent.spawn((
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
            ));

            // Online options container
            parent.append_menu_section()
                .with_children(|section| {
                    section.append_menu_button(
                        Color::srgb(0.3, 0.6, 0.9),
                        "Quick Match",
                        UINavSlot::row(0),
                    ).observe(on_quick_match);

                    section.append_menu_button(
                        Color::srgb(0.5, 0.4, 0.9),
                        "Create Room",
                        UINavSlot::row(1),
                    ).observe(on_create_room);

                    section.append_menu_button(
                        Color::srgb(0.9, 0.5, 0.3),
                        "Join Room",
                        UINavSlot::row(2),
                    ).observe(on_join_room);

                    section.append_menu_button(
                        Color::srgb(0.4, 0.7, 0.4),
                        "Friends List",
                        UINavSlot::row(3),
                    ).observe(on_friends_list);
                });

            // Back button
            parent.append_menu_button(
                Color::srgb(0.6, 0.6, 0.6),
                "Back",
                UINavSlot::row(4),
            ).observe(on_menu_to_main);
        });

        online_menu
    }

    fn spawn_settings_menu(&mut self) -> EntityCommands<'_> {
        let mut settings_menu = self.spawn_menu_base(MenuType::SettingsMenu);


        settings_menu.with_children(|parent| {

            parent.append_menu_title("SETTINGS");

            // Settings sections container
            parent.append_menu_section()
                .with_children(|section| {
                    // Audio Section Header
                    section.spawn((
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
                    ));

                    // Master Volume selector
                    section.append_slider(0.0, 100.0, 0.0, UINavSlot::row(0));


                    // SFX Volume selector
                    section.append_slider(0.0, 100.0, 0.0, UINavSlot::row(1));

                    // Graphics Section Header
                    section.spawn((
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
                    ));


                    section.append_selector(
                        vec![
                            UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                        ],
                        0,
                        UINavSlot::new(2, 0),
                        "Resolution".into(),
                    );

                    section.append_selector(
                        vec![
                            UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                            UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                            UIOption::screen("Exclusive", ScreenMode::ExclusiveFullScreen),
                        ],
                        0,
                        UINavSlot::new(3, 0),
                        "Fullscreen".into(),
                    );
                });

            // Back button
            parent.append_menu_button(
                Color::srgb(0.6, 0.6, 0.6),
                "Back",
                UINavSlot::row(4))
                .observe(handle_exit_button);
        });

        settings_menu
    }

    fn spawn_menu_base(&mut self, menu_type: MenuType) -> EntityCommands<'_> {

        self.spawn((
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
        ))

    }
}

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
    commands.spawn_offline_menu();
}

fn on_online(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<Menu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn_online_menu();
}

fn on_settings(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<Menu>>,
) {
    let entity = main_menu.single().expect("Main Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn_settings_menu();
}

fn on_exit(press: On<ButtonPressed>, mut exit: MessageWriter<AppExit>){
    exit.write(AppExit::Success);
}

fn on_settings_back(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    settings_menu: Query<Entity, With<Menu>>,
) {
    let entity = settings_menu.single().expect("Settings Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn_main_menu();
}

#[inline]
fn handle_exit_button(
    _press: On<ButtonPressed>,
    commands: Commands,
    query: Query<(Entity, &Menu)>,
) {
    handle_current_menu_exit(commands, query);
}


pub fn handle_current_menu_exit(
    mut commands: Commands,
    query: Query<(Entity, &Menu)>,
){
    if let Ok(menu) = query.single() {

        commands.entity(menu.0).despawn();

        match menu.1.menu_type {
            MenuType::MainMenu => {

            }
            _ =>{

            }
        }


    }
}

fn on_start_offline_game(
    _press: On<ButtonPressed>,
    mut commands: Commands,
){

}

fn on_menu_to_main(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    current_menu: Query<Entity, With<Menu>>,
){
    let entity = current_menu.single().expect("Menu doesn't exist");
    commands.entity(entity).despawn();
    commands.spawn_main_menu();
}
