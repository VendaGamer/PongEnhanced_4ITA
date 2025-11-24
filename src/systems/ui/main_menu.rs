use crate::components::ui::{MainMenu, MenuButton};
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
use bevy::ui::Pressed;
use leafwing_input_manager::prelude::ActionState;
use crate::resources::controls::MenuAction;
use crate::systems::ui::buttons::ButtonPressed;

const TITLE: &str = "PONG ENHANCED";
pub fn spawn_main_menu(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            MainMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                })
                .with_children(|title_parent| {
                    title_parent.spawn((
                        Text::new(TITLE),
                        TextFont {
                            font_size: 72.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 1.0)),
                    ));
                });


            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|button_container| {
                    // Offline Play Button
                    spawn_menu_button(
                        button_container,
                        "OFFLINE PLAY",
                        MenuButton::OfflinePlay,
                        Color::srgb(0.2, 0.6, 0.9),
                |on: On<ButtonPressed>| {
                            println!("PRESSED ONLINE PLAY");
                        }
                    );

                    // Online Play Button
                    spawn_menu_button(
                        button_container,
                        "ONLINE PLAY",
                        MenuButton::OnlinePlay,
                        Color::srgb(0.6, 0.3, 0.9),
                |on: On<ButtonPressed>| {
                            println!("PRESSED ONLINE PLAY");
                        }
                    );

                    // Settings Button
                    spawn_menu_button(
                        button_container,
                        "SETTINGS",
                        MenuButton::Settings,
                        Color::srgb(0.5, 0.5, 0.5),
                |on: On<ButtonPressed>| {
                            println!("PRESSED SETTINGS");
                        }
                    );

                    // Exit Game Button
                    spawn_menu_button(
                        button_container,
                        "EXIT GAME",
                        MenuButton::ExitGame,
                        Color::srgb(0.8, 0.2, 0.2),
                |on: On<ButtonPressed>, mut exit: MessageWriter<AppExit>| {
                            println!("PRESSED EXIT");
                            exit.write(AppExit::Success);
                        }
                    );
                });
        }).id()
}


fn spawn_menu_button<E: EntityEvent, B: Bundle, M>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    text: &str,
    button_type: MenuButton,
    color: Color,
    observer: impl IntoObserverSystem<E, B, M>,
) {
    parent
        .spawn((
            Button,
            button_type,
            Node {
                width: Val::Px(350.0),
                height: Val::Px(70.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
            BorderRadius::all(Val::Px(8.0)),
            BorderColor::from(Color::WHITE.with_alpha(0.3)),
        )).observe(observer)
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}


fn lighten_color(color: Color, amount: f32) -> Color {
    let [r, g, b, a] = color.to_srgba().to_f32_array();
    Color::srgba(
        (r + amount).min(1.0),
        (g + amount).min(1.0),
        (b + amount).min(1.0),
        a,
    )
}

pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}