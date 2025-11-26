use bevy::ecs::relationship::RelatedSpawnerCommands;
use crate::bundles::ui::LabelBundle;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::window::AppLifecycle;
use crate::systems::ButtonPressed;

#[derive(Bundle)]
pub struct ButtonBundle {
    button: Button,
    container: Node,
    background_color: BackgroundColor,
    border_radius: BorderRadius,
    border_color: BorderColor,
}

pub type MenuButton = (ButtonBundle, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);

impl ButtonBundle{
    pub fn menu_button(color: Color, text: &str) -> MenuButton {
        (
            Self{
                button: Button,
                container: Node {
                    width: Val::Px(350.0),
                    height: Val::Px(70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                background_color: BackgroundColor(color),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                border_color: BorderColor::from(Color::WHITE.with_alpha(0.3)),
            },
            children![LabelBundle::button_label(text)]
        )
    }

    pub fn spawn_main_menu_buttons(commands: &mut RelatedSpawnerCommands<ChildOf>) {
        commands.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(Val::Px(50.0),Val::Px(50.0),Val::Px(50.0),Val::ZERO),
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor::from(Color::srgb(0.1, 0.1, 0.1)),
        ))
            .with_children(|parent|{

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.2, 0.6, 0.9), "Offline Play"))
                    .observe(on_offline);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.6, 0.3, 0.9), "Online Play"))
                    .observe(on_offline);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.5, 0.5, 0.5), "Settings"))
                    .observe(on_settings);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.8, 0.2, 0.2), "Exit"))
                    .observe(on_exit);
            });
    }
}

fn on_offline(press: On<ButtonPressed>, commands: Commands){

}
fn on_online(press: On<ButtonPressed>, commands: Commands){

}

fn on_settings(press: On<ButtonPressed>, commands: Commands){

}

fn on_exit(press: On<ButtonPressed>, mut exit: MessageWriter<AppExit>){
    exit.write(AppExit::Success);
}