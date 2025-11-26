use crate::bundles::ui::LabelBundle;
use crate::components::ui::navigation::UINavSlot;
use crate::components::ui::HoverLight;
use crate::systems::ButtonPressed;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ButtonBundle {
    button: Button,
    container: Node,
    background_color: BackgroundColor,
    border_radius: BorderRadius,
    border_color: BorderColor,
    hover_light: HoverLight,
    navigation_slot: UINavSlot
}

pub type MenuButton = (ButtonBundle, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);

impl ButtonBundle{
    pub fn menu_button(color: Color, text: &str, slot: UINavSlot) -> MenuButton {
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
                hover_light: HoverLight {
                    amount: 0.0,
                    max: 0.3,
                    speed: 2.0,
                    base: color,
                },
                navigation_slot: slot,
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

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.2, 0.6, 0.9),
                               "Offline Play", UINavSlot::row(0)))
                    .observe(on_offline);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.6, 0.3, 0.9),
                               "Online Play", UINavSlot::row(1)))
                    .observe(on_offline);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.5, 0.5, 0.5),
                               "Settings", UINavSlot::row(2)))
                    .observe(on_settings);

                parent.spawn(ButtonBundle::menu_button(Color::srgb(0.8, 0.2, 0.2),
                               "Exit", UINavSlot::row(3)))
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