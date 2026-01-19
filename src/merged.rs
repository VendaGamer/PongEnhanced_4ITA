// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\area.rs --- 
use crate::bundles::paddle::PaddleBundle;
use crate::bundles::wall::WallBundle;
use crate::bundles::GoalBundle;
use crate::components::area::Area;
use crate::models::game::area::AreaShape;
use crate::utils::screen::TRANSFORM_ZERO;
use crate::utils::PADDLE_SIZE;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle {
    pub fn spawn(
        area_shape: &mut AreaShape,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        let teams = area_shape.get_teams_mut();

        for team in teams {
            let goal = commands.spawn(GoalBundle::new(team)).id();
            let positions = team.get_positions();

            for i in 0..team.players.len() {
                commands.spawn(PaddleBundle::new(meshes, materials, positions[i],
                                                 PADDLE_SIZE, goal, team.players[i].entity));
            }
            
            team.area_side.spawn_score_text(commands);
        }

        let walls = area_shape.get_wall_sides();
        for side in walls {
            commands.spawn(WallBundle::new(*side));
        }
        
        commands.spawn(AreaBundle {
            transform: TRANSFORM_ZERO,
            area: Area,
            global_transform: GlobalTransform::from(TRANSFORM_ZERO)
        });
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\ball.rs --- 
use crate::utils::screen::ZERO_DAMPING;
use crate::Ball;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::*;
use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub collider: Collider,
    pub friction: Friction,
    pub restitution: Restitution,
    pub damping: LinearDamping,
    pub collision_layers: CollisionLayers,
    pub gravity_scale: GravityScale,
    pub collision_events_enabled: CollisionEventsEnabled,
    pub ccd: SweptCcd,
}

impl BallBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        initial_velocity: Vec2,
        radius: f32,
    ) -> Self {
        Self {
            ball: Ball{
                initial_velocity
            },
            mesh: Mesh2d(meshes.add(Circle::new(radius))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Dynamic,
            linear_velocity: LinearVelocity(initial_velocity),
            angular_velocity: AngularVelocity(0.0),
            collider: Collider::circle(radius),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            damping: ZERO_DAMPING,
            collision_layers: CollisionLayers::default(),
            gravity_scale: GravityScale(0.0),
            collision_events_enabled: CollisionEventsEnabled,
            ccd: SweptCcd::default(),
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\camera.rs --- 
use crate::utils::FIXED_DIMENSIONS;
use bevy::camera;
use bevy::color::Color;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera2d: Camera2d,
    pub camera: Camera,
    pub projection: Projection,
    pub ui_anti_alias: UiAntiAlias,
    pub msaa: Msaa,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            camera2d: Camera2d,
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: camera::ScalingMode::Fixed {
                    width: FIXED_DIMENSIONS.x,
                    height: FIXED_DIMENSIONS.y,
                },
                ..OrthographicProjection::default_2d()
            }.into(),
            ui_anti_alias: UiAntiAlias::Off,
            msaa: Msaa::Off,
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\division_line.rs --- 
use bevy::prelude::*;
use crate::components::DivisionLine;

#[derive(Bundle)]
pub struct DivisionLineBundle {
    pub division_line: DivisionLine,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

impl DivisionLineBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        const LINE_WIDTH: f32 = 4.0;
        const SEGMENT_HEIGHT: f32 = 20.0;
        const GAP_HEIGHT: f32 = 15.0;
        const TOTAL_HEIGHT: f32 = 720.0;

        Self {
            division_line: DivisionLine,
            mesh: Mesh2d(meshes.add(Rectangle::new(LINE_WIDTH, SEGMENT_HEIGHT))),
            material: MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.5))),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\goal.rs --- 
use crate::components::Goal;
use crate::models::game::area::TeamInfo;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Bundle, Transform};

#[derive(Bundle)]
pub struct GoalBundle {
    pub goal: Goal,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl GoalBundle{
    pub fn new(team: &TeamInfo) -> Self{

        Self{
            goal: Goal{
                side: team.area_side,
            },
            collider: team.area_side.get_collider(),
            transform: team.area_side.get_transform(),
            rigid_body: RigidBody::Static
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\mod.rs --- 
pub mod paddle;
pub mod ball;
pub mod camera;
pub mod area;
pub mod goal;
pub mod wall;
pub mod division_line;

pub use ball::*;
pub use bevy::prelude::*;
pub use camera::*;
pub use division_line::*;
pub use goal::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\paddle.rs --- 
use crate::Paddle;
use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
}

impl PaddleBundle {

    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        size: Vec2,
        goal: Entity,
        player: Entity
    ) -> Self {
        Self {
            paddle: Paddle{
                goal,
                player
            },
            mesh: Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(size.x, size.y),
            restitution: Restitution::new(0.0),
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\gameplay\wall.rs --- 
use avian2d::prelude::{Collider, Restitution, RigidBody};
use bevy::prelude::Transform;
use crate::bundles::Bundle;
use crate::components::*;
use crate::models::game::area::AreaSide;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub restitution: Restitution
}

impl WallBundle {
    pub fn new(side: AreaSide) -> Self {
        let collider = AreaSide::get_collider(side);
        let transform = AreaSide::get_transform(side);

        Self {
            wall: Wall {
                side
            },
            collider,
            transform,
            rigid_body: RigidBody::Static,
            restitution: Restitution::new(0.0)
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\mod.rs --- 
pub mod player;
pub mod gameplay;
pub mod ui;

pub use gameplay::*;
pub use ui::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\player.rs --- 
use crate::components::Player;
use bevy::prelude::Bundle;
use leafwing_input_manager::input_map::InputMap;
use crate::resources::PlayerAction;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub bindings: InputMap<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(id: u8) -> Self {
        Self {
            player: Player {
                id
            },
            bindings: Player::get_default_input_map(id),
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\ui\mod.rs --- 
pub mod widgets; 
// --- C:\Projects\PongEnhanced_4ITA\src\bundles\ui\widgets.rs --- 
use crate::bundles::{children, default, BackgroundColor, BorderRadius, Bundle, ChildOf, Color, Node, Spawn, Text, TextColor, TextFont, UiRect, Val};
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::text::FontSmoothing;


const GAME_TITLE: &'static str = "PONG ENHANCED";

#[derive(Bundle)]
pub struct LabelBundle;

impl LabelBundle {
    pub fn button_label(text: impl Into<String>) -> impl Bundle {
        (
            Text::new(text),
            TextFont {
                font_size: 32.0,
                font_smoothing: FontSmoothing::None,
                ..default()
            },
            TextColor(Color::WHITE),
        )
    }

    pub fn game_title() -> impl Bundle {
        (
            Node{
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
            Children::spawn_one((
                Text::new(GAME_TITLE),
                TextFont {
                    font_size: 72.0,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ))
        )
    }

    pub fn custom(text: &str, color: Color, size: f32) -> impl Bundle {
        (
            Text::new(text),
            TextFont {
                font_size: size,
                font_smoothing: FontSmoothing::None,
                ..default()
            },
            TextColor(color),
        )
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\gameplay\area.rs --- 
use crate::models::game::area::AreaSide;
use bevy::prelude::*;

#[derive(Component)]
pub struct Area;

#[derive(Component)]
pub struct DivisionLine;

#[derive(Component)]
pub struct Goal {
    pub side: AreaSide,
}

#[derive(Component)]
pub struct Wall {
    pub side: AreaSide,
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\gameplay\game.rs --- 
use bevy::prelude::Component;
use crate::bundles::{App, Entity, Vec2};

#[derive(Component)]
pub struct Ball {
    pub initial_velocity: Vec2
}

pub trait GameModeRules: Send + Sync {
    fn ball_speed(&self) -> f32;
    fn gravity_scale(&self) -> f32;
    fn paddle_speed_multiplier(&self) -> f32;
    fn apply_special_mechanics(&self, app: &mut App);
}

#[derive(Component, Copy, Clone)]
pub struct Paddle {
    pub goal: Entity,
    pub player: Entity,
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\gameplay\game_modes.rs --- 
use crate::bundles::Component;

#[derive(Component)]
pub struct PlayerHealth{
    player_health: i32
}

pub const PUSH_COOLDOWN: f32 = 5.0;

#[derive(Component)]
pub struct PaddlePush {
    pub current_cooldown: f32,
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\gameplay\mod.rs --- 
pub mod area;
pub mod game;
pub mod game_modes;


pub use area::*;
pub use game::*;
 
// --- C:\Projects\PongEnhanced_4ITA\src\components\mod.rs --- 
pub mod gameplay;
pub mod player;
pub mod ui;

pub use gameplay::*;
pub use player::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\components\player.rs --- 
use crate::resources::controls::PlayerAction;
use bevy::prelude::{Component, KeyCode};
use leafwing_input_manager::prelude::InputMap;

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

impl Player {
    pub fn get_default_input_map(id: u8) -> InputMap<PlayerAction> {
        match id {
            1 => InputMap::new([
                (PlayerAction::Up, KeyCode::KeyW),
                (PlayerAction::Down, KeyCode::KeyS),
                (PlayerAction::Right, KeyCode::KeyD),
                (PlayerAction::Left, KeyCode::KeyA),
                (PlayerAction::Dash, KeyCode::ControlLeft),
                (PlayerAction::Push, KeyCode::Space),
                (PlayerAction::Pause, KeyCode::Escape),
                (PlayerAction::Speedup, KeyCode::ShiftLeft),
            ]),
            2 => InputMap::new([
                (PlayerAction::Up, KeyCode::ArrowUp),
                (PlayerAction::Down, KeyCode::ArrowDown),
                (PlayerAction::Right, KeyCode::ArrowRight),
                (PlayerAction::Left, KeyCode::ArrowLeft),
                (PlayerAction::Dash, KeyCode::ControlRight),
                (PlayerAction::Push, KeyCode::Enter),
                (PlayerAction::Pause, KeyCode::End),
                (PlayerAction::Speedup, KeyCode::ShiftRight),
            ]),
            3 => InputMap::new([
                (PlayerAction::Up, KeyCode::KeyI),
                (PlayerAction::Down, KeyCode::KeyK),
                (PlayerAction::Right, KeyCode::KeyL),
                (PlayerAction::Left, KeyCode::KeyJ),
                (PlayerAction::Dash, KeyCode::KeyO),
                (PlayerAction::Push, KeyCode::KeyU),
                (PlayerAction::Pause, KeyCode::KeyP),
                (PlayerAction::Speedup, KeyCode::KeyB),
            ]),
            4 => InputMap::new([
                (PlayerAction::Up, KeyCode::Numpad8),
                (PlayerAction::Down, KeyCode::Numpad5),
                (PlayerAction::Right, KeyCode::Numpad6),
                (PlayerAction::Left, KeyCode::Numpad4),
                (PlayerAction::Dash, KeyCode::NumpadComma),
                (PlayerAction::Push, KeyCode::NumpadEnter),
                (PlayerAction::Pause, KeyCode::NumpadDivide),
                (PlayerAction::Speedup, KeyCode::Numpad0),
            ]),
            _ => InputMap::default(),
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\ui\effects.rs --- 
use bevy::prelude::Entity;
use crate::bundles::{Color, Component};
use crate::utils::lighten_color;

#[derive(Component)]
pub struct HoverLight(pub Color);

#[derive(Component)]
pub struct HoverLightColor {
    pub hover_color: Color,
}


impl HoverLightColor {
    pub fn new(base_color: Color, lighten_amount: f32) -> Self {
        Self {
            hover_color: lighten_color(base_color, lighten_amount),
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\ui\in_game.rs --- 
use crate::bundles::Component;
use crate::models::game::area::AreaSide;

#[derive(Component)]
pub struct ScoreText {
    pub area_side: AreaSide,
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\ui\menu.rs --- 
use std::iter::Map;
use bevy::prelude::Entity;
use crate::bundles::Component;

#[derive(Component)]
pub struct Menu{
    pub selectables: Option<Map<Entity, Entity>>,
    pub menu_type: MenuType,
}

impl Menu{
    pub fn new(menu_type: MenuType) -> Self{
        Self{
            selectables: None,
            menu_type
        }
    }

}

pub enum MenuType {
    MainMenu,
    SettingsMenu,
    OfflinePlayMenu,
    OnlinePlayMenu
} 
// --- C:\Projects\PongEnhanced_4ITA\src\components\ui\mod.rs --- 
pub mod menu;
pub mod widgets;
pub mod in_game;
pub mod effects;

pub use menu::*;
pub use widgets::*;
pub use in_game::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\components\ui\widgets.rs --- 
use bevy::prelude::*;
use derive_more::{From, Into};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub enum SourceHandle<T: 'static + ?Sized> {
    Strong(Arc<T>),
    Static(&'static T),
    Unique(Box<T>)
}

impl<T: ?Sized> SourceHandle<T> {
    fn get_ref(&self) -> &T {
        match self {
            SourceHandle::Strong(arc) => arc.as_ref(),
            SourceHandle::Static(r) => r,
            SourceHandle::Unique(boxed) => boxed.as_ref()
        }
    }
}

pub trait UIOptionProvider: Send + Sync + Any {
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait UIOptionValue: Any + Send + Sync + Debug + UIOptionString {

    fn as_any(&self) -> &dyn Any;
}

impl<T> UIOptionValue for T
where
    T: Any + Send + Sync + Debug + UIOptionString
{
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: UIOptionValue> UIOptionProvider for Vec<T> {

    #[inline]
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue> {
        self.get(index).map(|val| val as &dyn UIOptionValue)
    }
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: UIOptionValue, const N: usize> UIOptionProvider for [T; N]  {
    #[inline]
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue> {
        self.get(index).map(|val| val as &dyn UIOptionValue)
    }
    #[inline]
    fn len(&self) -> usize {
        N
    }
}

impl<T: UIOptionValue + 'static> UIOptionProvider for [T] {
    #[inline]
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue> {
        self.get(index).map(|val| val as &dyn UIOptionValue)
    }
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}


#[derive(Component)]
pub struct Dropdown {
    pub selected: usize,
    pub options: Arc<dyn UIOptionProvider>,
}
#[derive(Component)]
#[require(Button)]
pub struct SelectorButton(pub bool);

#[derive(Component)]
#[require(Text)]
pub struct SelectorText;

#[derive(Component, From, Into)]
pub struct OptionSelector {
    pub selected: usize,
    pub options_provider: SourceHandle<dyn UIOptionProvider>,
}

pub trait UIOptionString {
    fn push_ui_option_string(&self, string: &mut String);
}

#[derive(Component)]
pub enum SettingsSelector {
    WindowMode,
    Monitor,
    Resolution,
    RefreshRate,
    ShowFPS,
}

impl OptionSelector {

    pub fn current<T: Any>(&self) -> Option<&T> {
        self.options_provider
            .get_ref()
            .get_option(self.selected)?
            .as_any()
            .downcast_ref::<T>()
    }

    pub fn push_current_string(&self, string: &mut String) {
        if let Some(current) = self.options_provider.get_ref().get_option(self.selected) {
            current.push_ui_option_string(string);
            return;
        }

        string.push_str("n/a");
    }

    pub fn next(&mut self) {
        let provider = self.options_provider.get_ref();

        if !provider.is_empty() {
            self.selected = (self.selected + 1) % provider.len();
        }
    }

    pub fn prev(&mut self) {
        let provider = self.options_provider.get_ref();

        if !provider.is_empty() {
            self.selected = if self.selected == 0 {
                provider.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn set(&mut self, idx: usize) {
        if idx < self.options_provider.get_ref().len() {
            self.selected = idx;
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\events\mod.rs --- 
pub mod widgets; 
// --- C:\Projects\PongEnhanced_4ITA\src\events\widgets.rs --- 
use bevy::prelude::{Entity, EntityEvent};

#[derive(EntityEvent)]
pub struct ButtonPressed(pub Entity);

#[derive(EntityEvent)]
pub struct SelectorValueChanged(pub Entity);

#[derive(EntityEvent)]
pub struct OptionChanged(pub Entity);

#[derive(EntityEvent)]
pub struct SliderValueChanged{
    pub entity: Entity,
    pub value: f32,
} 
// --- C:\Projects\PongEnhanced_4ITA\src\lib.rs --- 
mod components;
mod resources;
mod systems;
mod bundles;
mod plugins;
mod utils;
mod events;
mod models;
mod traits;

use crate::plugins::game_ui::GameUIPlugin;
use crate::plugins::GameCorePlugin;
use crate::resources::controls::PlayerAction;
use crate::resources::MenuAction;
use crate::systems::settings::persistence::load_settings;
use crate::utils::DEFAULT_FONT;
use avian2d::prelude::*;
use bevy::input_focus::InputDispatchPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{Backends, PowerPreference, RenderCreation, WgpuSettings, WgpuSettingsPriority};
use bevy::render::RenderPlugin;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::PresentMode;
use bevy_tween::DefaultTweenPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use wgpu_types::Limits;

fn main() {
    let mut app = App::new();

    let settings = load_settings();

    app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        mode: settings.video_mode,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
            .disable::<bevy::pbr::PbrPlugin>()
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(
                    WgpuSettings {
                        backends: Some(Backends::all()),
                        priority: WgpuSettingsPriority::Functionality,
                        power_preference: PowerPreference::HighPerformance,
                        features: WgpuFeatures::default()
                            .difference(WgpuFeatures::VERTEX_WRITABLE_STORAGE),
                        limits: Limits {
                            max_storage_buffers_per_shader_stage: 10,
                            ..default()
                        },
                        ..default()
                    }
                ),
                ..default()
            }),
            PhysicsPlugins::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuAction>::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            DefaultTweenPlugins,

            // my plugins
            GameCorePlugin,
            GameUIPlugin,
        ))
        .insert_resource(settings);

    let world = app.world_mut();

    world.resource_mut::<Assets<_>>()
        .insert(AssetId::default(), Font::try_from_bytes(DEFAULT_FONT.into())
        .unwrap())
        .expect("UNABLE TO LOAD FONT");


    app.run();
}
 
// --- C:\Projects\PongEnhanced_4ITA\src\main.rs --- 
mod components;
mod resources;
mod systems;
mod bundles;
mod plugins;
mod utils;
mod events;
mod models;
mod traits;

use crate::plugins::game_ui::GameUIPlugin;
use crate::plugins::GameCorePlugin;
use crate::resources::controls::PlayerAction;
use crate::resources::MenuAction;
use crate::systems::settings::persistence::load_settings;
use crate::utils::DEFAULT_FONT;
use avian2d::prelude::*;
use bevy::input_focus::InputDispatchPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{Backends, PowerPreference, RenderCreation, WgpuSettings, WgpuSettingsPriority};
use bevy::render::RenderPlugin;
use bevy::ui_widgets::UiWidgetsPlugins;
use bevy::window::PresentMode;
use bevy_tween::DefaultTweenPlugins;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use wgpu_types::Limits;

fn main() {
    let mut app = App::new();

    let settings = load_settings();

    app.add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        mode: settings.video_mode,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
            .disable::<bevy::pbr::PbrPlugin>()
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(
                    WgpuSettings {
                        backends: Some(Backends::all()),
                        priority: WgpuSettingsPriority::Functionality,
                        power_preference: PowerPreference::HighPerformance,
                        features: WgpuFeatures::default()
                            .difference(WgpuFeatures::VERTEX_WRITABLE_STORAGE),
                        limits: Limits {
                            max_storage_buffers_per_shader_stage: 10,
                            ..default()
                        },
                        ..default()
                    }
                ),
                ..default()
            }),
            PhysicsPlugins::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<MenuAction>::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            DefaultTweenPlugins,

            // my plugins
            GameCorePlugin,
            GameUIPlugin,
        ))
        .insert_resource(settings);

    let world = app.world_mut();

    world.resource_mut::<Assets<_>>()
        .insert(AssetId::default(), Font::try_from_bytes(DEFAULT_FONT.into())
        .unwrap())
        .expect("UNABLE TO LOAD FONT");


    app.run();
}
 
// --- C:\Projects\PongEnhanced_4ITA\src\models\game\area.rs --- 
use crate::bundles::widgets::LabelBundle;
use crate::bundles::{default, Entity, Transform, Vec3};
use crate::components::ui::{ScoreText, UIOptionString};
use crate::utils::{FIXED_DIMENSIONS, HALF_HEIGHT, HALF_WALL_THICKNESS, HALF_WIDTH, WALL_THICKNESS};
use avian2d::prelude::Collider;
use bevy::prelude::{Color, Commands, Node, PositionType, Resource};
use bevy::ui::Val;
use serde::{Deserialize, Serialize};
use AreaShape::{Cuboid, Triangular, TwoSide};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub enum AreaSide {
    Left,
    Right,
    Top,
    Bottom,
}


impl AreaSide{
    pub fn opposite(self) -> AreaSide {
        match self {
            AreaSide::Left => AreaSide::Right,
            AreaSide::Right => AreaSide::Left,
            AreaSide::Top => AreaSide::Bottom,
            AreaSide::Bottom => AreaSide::Top,
        }
    }

    pub fn spawn_score_text(self, commands: &mut Commands){
        let position = match self {
            AreaSide::Left => Vec3::new(-HALF_WIDTH / 2.0, 0.0, 0.0),
            AreaSide::Right => Vec3::new(HALF_WIDTH / 2.0, 0.0, 0.0),
            AreaSide::Top => Vec3::new(0.0, HALF_HEIGHT / 2.0, 0.0),
            AreaSide::Bottom => Vec3::new(0.0, -HALF_HEIGHT / 2.0, 0.0),
        };

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                ..default()
            },
            LabelBundle::custom("0", Color::WHITE.into(), 80.0),
            ScoreText {
                area_side: self,
            }
        ));
    }

    pub fn is_vertical(self) -> bool {
        match self {
            AreaSide::Left | AreaSide::Right => true,
            AreaSide::Top | AreaSide::Bottom => false,
        }
    }
}


#[derive(Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct TeamInfo {
    pub current_score: u32,
    pub area_side: AreaSide,
    pub players: Vec<PlayerInfo>,
}

impl TeamInfo {
    pub fn get_positions(&self) -> Vec<Vec3> {
        self.area_side.get_paddle_pos(self.players.len())
    }
}

#[derive(Resource, Default)]
pub struct Players {
    pub players: Vec<PlayerInfo>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub entity: Entity,
}

pub enum ControlType{
    Keyboard,
    Gamepad(Entity),
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub enum AreaShape {
    TwoSide([TeamInfo; 2]),
    Triangular([TeamInfo; 3]),
    Cuboid([TeamInfo; 4]),
}

impl AreaShape {
    pub(crate) fn default() -> AreaShape {

        TwoSide([
            TeamInfo {
                current_score: 0,
                area_side: AreaSide::Left,
                players: Vec::new(),
            },
            TeamInfo {
                current_score: 0,
                area_side: AreaSide::Right,
                players: Vec::new(),
            },
        ])

    }
}

impl UIOptionString for AreaShape {
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match self {
            TwoSide(_) => "Two Side",
            Triangular(_) => "Triangular",
            Cuboid(_) => "Cuboid",
        };
        string.push_str(s);
    }
}


impl AreaShape {


    pub fn get_wall_sides(&self) -> &[AreaSide] {
        match self {
            TwoSide(_) => &[AreaSide::Top, AreaSide::Bottom],
            Triangular(_) => &[],
            Cuboid(_) => &[],
        }
    }
    pub fn get_team(&mut self, side: AreaSide) -> Option<&TeamInfo> {

        let teams = self.get_teams();

        for team in teams.iter(){
            if side == team.area_side {
                return Some(team);
            }
        }
        None

    }

    pub fn get_team_mut(&mut self, side: AreaSide) -> Option<&mut TeamInfo> {

        let teams = self.get_teams_mut();

        for team in teams.iter_mut(){
            if side == team.area_side {
                return Some(team);
            }
        }
        
        None
    }

    pub fn get_teams(&self) -> &[TeamInfo] {
        match self {
            TwoSide(opt)     => opt.as_ref(),
            Triangular(opt)  => opt.as_ref(),
            Cuboid(opt)      => opt.as_ref(),
        }
    }

    pub fn get_teams_mut(&mut self) -> &mut [TeamInfo] {
        match self {
            TwoSide(opt)     => opt.as_mut(),
            Triangular(opt)  => opt.as_mut(),
            Cuboid(opt)      => opt.as_mut(),
        }
    }
}

impl AreaSide {

    pub fn get_transform(self) -> Transform {


        match self {
            AreaSide::Left => Transform {
                translation: Vec3::new(-HALF_WIDTH - HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            AreaSide::Right => Transform {
                translation: Vec3::new(HALF_WIDTH + HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            AreaSide::Top => Transform {
                translation: Vec3::new(0.0, HALF_HEIGHT + HALF_WALL_THICKNESS, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
            AreaSide::Bottom => Transform {
                translation: Vec3::new(0.0, -HALF_HEIGHT - HALF_WALL_THICKNESS / 2.0, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
        }
    }

    pub fn get_paddle_pos(self, player_count: usize) -> Vec<Vec3> {
        if player_count == 0 {
            return Vec::new();
        }

        let mut positions = Vec::with_capacity(player_count);

        match self {
            AreaSide::Left | AreaSide::Right => {
                let x = if self == AreaSide::Left {
                    -HALF_WIDTH + 50.0
                } else {
                    HALF_WIDTH - 50.0
                };

                // Evenly distribute along the height
                let spacing = FIXED_DIMENSIONS.y / (player_count + 1) as f32;

                for i in 0..player_count {
                    let y = -HALF_HEIGHT + spacing * (i + 1) as f32;
                    positions.push(Vec3::new(x, y, 0.0));
                }
            },
            AreaSide::Top | AreaSide::Bottom => {
                let y = if self == AreaSide::Top {
                    HALF_HEIGHT - 50.0
                } else {
                    -HALF_HEIGHT + 50.0
                };

                // Evenly distribute along the width
                let spacing = FIXED_DIMENSIONS.x / (player_count + 1) as f32;

                for i in 0..player_count {
                    let x = -HALF_WIDTH + spacing * (i + 1) as f32;
                    positions.push(Vec3::new(x, y, 0.0));
                }
            }
        }

        positions
    }

    pub fn get_collider(self) -> Collider {
        match self {
            AreaSide::Left | AreaSide::Right => {
                Collider::rectangle(WALL_THICKNESS, FIXED_DIMENSIONS.y)
            }
            AreaSide::Top | AreaSide::Bottom => {
                Collider::rectangle(FIXED_DIMENSIONS.x, WALL_THICKNESS)
            }
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\models\game\gameplay.rs --- 
use std::fmt::Write;
use bevy::prelude::DerefMut;
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Serialize};
use crate::components::ui::{UIOptionString};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize, Default, Debug)]
pub enum GameMode {
    #[default]
    Classic,
    UpsideDown,
    Modern,
    Blackout,
    Twisted,
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq, Serialize,
         Deserialize, Default, From, Into, Deref, DerefMut)]
pub struct PlayerNum(pub u8);

impl UIOptionString for PlayerNum {
    fn push_ui_option_string(&self, string: &mut String) {
        write!(string, "{} Players", self.0).unwrap();
    }
}

impl UIOptionString for GameMode {
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match self {
            GameMode::Classic => "Classic",
            GameMode::UpsideDown => "Upside Down",
            GameMode::Modern => "Modern",
            GameMode::Blackout => "Blackout",
            GameMode::Twisted => "Twisted",
        };

        string.push_str(s);
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\models\game\mod.rs --- 
pub mod area;
pub mod gameplay; 
// --- C:\Projects\PongEnhanced_4ITA\src\models\mod.rs --- 
pub mod ui;
pub mod game; 
// --- C:\Projects\PongEnhanced_4ITA\src\models\ui\mod.rs --- 
pub mod option; 
// --- C:\Projects\PongEnhanced_4ITA\src\models\ui\option.rs --- 
use std::fmt::Debug;
use bevy::window::PresentMode;
use crate::components::ui::{SourceHandle, UIOptionString};


pub const VSyncOptions: SourceHandle<[PresentMode]> = 
SourceHandle::Static
(&[
    PresentMode::AutoNoVsync,
    PresentMode::AutoVsync
]);

impl UIOptionString for PresentMode {
    fn push_ui_option_string(&self, string: &mut String) {

        let s = match *self {
            PresentMode::AutoVsync => "Vsync On",
            PresentMode::Fifo => "Vsync On",
            PresentMode::FifoRelaxed => "Adaptive Vsync",
            PresentMode::Immediate => "Vsync Off",
            PresentMode::Mailbox => "Fast Vsync",
            PresentMode::AutoNoVsync => "Vsync Off",
        };

        string.push_str(s);
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\plugins\game_core.rs --- 
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::models::game::area::{PlayerInfo, Players};
use crate::resources::controls::MenuAction;
use crate::resources::{GameModeConfig, GameSettings, Monitors};
use crate::systems::menu::m_main;
use crate::systems::selectors::update_selector_text;
use crate::systems::settings::monitor::on_spawn_monitors;
use crate::systems::*;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_paddle,
                check_connection,
                maintain_ball_speed,
                paddle_hit_dynamics,
                update_score_ui,
                update_selector_text,
            ))
            .add_systems(Startup, (
                setup_common,
            ))
            .add_systems(PostStartup, (
                on_spawn_monitors,
            ))
            .insert_resource(GameModeConfig::default())
            .insert_resource(Players::default());
    }
}

fn setup_common(
    mut commands: Commands,
    mut players: ResMut<Players>
) {
    commands.spawn(CameraBundle::default());

    for i in 0..4{
        let entity = commands.spawn(PlayerBundle::new(i)).id();
        let info = PlayerInfo {
            entity,
            name: format!("Player {}", i + 1),
        };
        players.players.push(info);
    }

    commands.spawn(MenuAction::input_map());
    commands.spawn(m_main());
} 
// --- C:\Projects\PongEnhanced_4ITA\src\plugins\game_ui.rs --- 
﻿use crate::bundles::{App, Commands, MessageReader, On, Plugin, ResMut, UiScale, Update};
use crate::events::widgets::SliderValueChanged;
use crate::systems::widgets::*;
use bevy::prelude::Query;
use bevy::ui_widgets::{SliderValue, ValueChange};
use bevy::window::WindowResized;
use crate::utils::FIXED_DIMENSIONS;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                u_ui_hover_light,
                u_slider_visuals,
                t_button_press,
                handle_ui_scaling,
                ))
            .add_observer(t_slider_change)
            .add_observer(update_selector);
    }
}

fn handle_ui_scaling(
    mut ui_scale: ResMut<UiScale>,
    mut resized: MessageReader<WindowResized>)
{
    for event in resized.read() {

        let scale_x = event.width / FIXED_DIMENSIONS.x;
        let scale_y = event.height / FIXED_DIMENSIONS.y;

        let scale = scale_y.min(scale_x);

        ui_scale.0 = scale;
    }
}

fn t_slider_change(
    value_change: On<ValueChange<f32>>,
    mut commands: Commands,
    sliders: Query<&SliderValue>)
{
    if let Ok(slider_val) = sliders.get(value_change.source){
        if slider_val.0 == value_change.value{
            return;
        }

        commands
            .entity(value_change.source)
            .insert(SliderValue(value_change.value));

        commands.trigger(SliderValueChanged {
            entity: value_change.source,
            value: value_change.value,
        });
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\plugins\mod.rs --- 
pub mod game_core;
pub mod game_ui;

pub use game_core::*;

 
// --- C:\Projects\PongEnhanced_4ITA\src\resources\game\controls.rs --- 
use bevy::prelude::{GamepadButton};
use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::prelude::*;
use crate::bundles::KeyCode;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Speedup,
    Dash,
    Push,
    Pause
}

#[derive(Actionlike, Clone, Copy, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum MenuAction {
    Confirm,
    Cancel,
    #[actionlike(DualAxis)]
    Navigate
}

impl MenuAction{
    pub fn input_map() -> InputMap<Self>{
        let mut map = InputMap::default();

        map.insert(MenuAction::Confirm, KeyCode::Enter);
        map.insert(MenuAction::Confirm, KeyCode::Space);
        map.insert(MenuAction::Cancel, KeyCode::Escape);

        map.insert(MenuAction::Confirm, GamepadButton::South);
        map.insert(MenuAction::Cancel, GamepadButton::East);


        map.insert_dual_axis(
            MenuAction::Navigate,
            VirtualDPad::wasd(),
        );
        map.insert_dual_axis(
            MenuAction::Navigate,
            VirtualDPad::arrow_keys(),
        );
        
        map.insert_dual_axis(
            MenuAction::Navigate,
            GamepadStick::LEFT,
        );
        map.insert_dual_axis(
            MenuAction::Navigate, 
            VirtualDPad::dpad(),
        );

        map
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\resources\game\game_config.rs --- 
use crate::components::ui::UIOptionString;
use crate::models::game::area::AreaShape;
use crate::models::game::gameplay::GameMode;
use bevy::prelude::{Deref, Res, Resource, UVec2};
use bevy::window::{MonitorSelection, PresentMode, VideoMode, WindowMode};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub video_mode: WindowMode,
    pub vsync: PresentMode,
    pub show_fps: bool,
}

#[derive(Resource, Clone, Eq, PartialEq, Debug)]
pub struct PendingSettings {
    pub video_mode: WindowMode,
    pub vsync: PresentMode,
    pub show_fps: bool,
}

impl From<&Res<'_, GameSettings>> for PendingSettings {
    fn from(settings: &Res<'_, GameSettings>) -> Self {
        Self{
            video_mode: settings.video_mode,
            vsync: settings.vsync,
            show_fps: settings.show_fps,
        }
    }
}



#[derive(Resource, Clone, Default, Debug)]
pub struct Monitors {
    pub monitors: Arc<Vec<MonitorInfo>>,
    pub selected_monitor: usize,
}

impl UIOptionString for MonitorInfo {
    fn push_ui_option_string(&self, string: &mut String){
        string.push_str(&*self.name);
    }
}


impl Monitors {
    pub fn get_current_monitor(&self) -> &MonitorInfo {
        self.monitors.get(self.selected_monitor).expect("no monitor found")
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonitorInfo {
    pub monitor_selection: MonitorSelection,
    pub native_mode: VideoMode,
    pub name: String,
    pub refresh_rates: Arc<Vec<RefreshRate>>,
    pub resolutions: Arc<Vec<Resolution>>,
    pub bit_depths: Arc<Vec<BitDepth>>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct RefreshRate(pub u32);

impl UIOptionString for RefreshRate {

    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{} Hz", self.0/1000).as_str());
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, From, Into, Deref)]
pub struct Resolution(pub UVec2);

impl UIOptionString for Resolution {
    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{} x {}", self.0.x, self.0.y).as_str());
    }
}
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct BitDepth(pub u16);

impl UIOptionString for BitDepth {
    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{}-bit", self.0).as_str())
    }
}


impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 50.0,
            sfx_volume: 50.0,
            video_mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            vsync: PresentMode::AutoVsync,
            show_fps: true,
        }
    }
}

#[derive(Resource, Hash, PartialEq, Eq)]
pub struct GameModeConfig {
    pub game_mode: GameMode,
    pub area_shape: AreaShape,
    pub win_score: u32,
}

impl Default for GameModeConfig {
    fn default() -> Self {
        Self{
            game_mode: GameMode::Classic,
            area_shape: AreaShape::default(),
            win_score: 10,
        }
    }
}

impl GameModeConfig {

    pub fn get_ball_speed(&self) -> f32 {
        match self.game_mode {
            GameMode::Classic => 400.0,
            GameMode::Modern => 600.0,
            GameMode::Twisted => 450.0,
            _ => 400.0,
        }
    }

    pub fn get_paddle_speed(&self) -> f32 {
        match self.game_mode {
            GameMode::Classic => 400.0,
            GameMode::UpsideDown => 300.0,
            GameMode::Modern => 500.0,
            _ => 400.0,
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\resources\game\mod.rs --- 
pub mod controls;
pub mod game_config;
pub use game_config::*;
pub use controls::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\resources\mod.rs --- 
pub mod game;
pub use game::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\gameplay\handle_gamepads.rs --- 
use crate::components::Player;
use crate::resources::PlayerAction;
use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent};
use bevy::prelude::{MessageReader, Query, With};
use leafwing_input_manager::input_map::InputMap;

pub fn check_connection(
    mut events: MessageReader<GamepadConnectionEvent>,
    mut bindings: Query<&InputMap<PlayerAction>, With<Player>>,
) {
    for ev in events.read() {

        if let GamepadConnection::Connected{ .. } = &ev.connection {

            println!("Connected Gamepad");

        }

    }
}
 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\gameplay\handle_scoring.rs --- 
use crate::bundles::BallBundle;
use crate::components::ui::ScoreText;
use crate::components::*;
use crate::resources::GameModeConfig;
use crate::utils::screen::BALL_RADIUS;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn handle_scoring(
    collision: On<CollisionStart>,
    goals: Query<&Goal>,
    mut game_config: ResMut<GameModeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let ball = collision.collider1;
    let other = collision.collider2;

    if let Ok(goal) = goals.get(other) {
        if let Some(team) = game_config.area_shape.get_team_mut(goal.side){

            team.current_score += 1;

            commands.entity(ball).despawn();

            commands.spawn(BallBundle::new(
                &mut meshes,
                &mut materials,
                Vec3::ZERO,
                Vec2::new(-300.0, 300.0),
                BALL_RADIUS
            )).observe(handle_scoring);

        }
    }
}

pub fn update_score_ui(
    mut game_config: ResMut<GameModeConfig>,
    mut score_texts: Query<(&mut Text, &ScoreText)>,
) {
    if !game_config.is_changed(){
        return;
    }

    for (mut text, score_text) in score_texts.iter_mut() {
        if let Some(team) = game_config.area_shape.get_team(score_text.area_side){
            text.0 = team.current_score.to_string();
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\gameplay\mod.rs --- 
pub mod movement;
pub mod handle_gamepads;
pub mod handle_scoring;

pub use movement::*;
pub use handle_scoring::*;
pub use handle_gamepads::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\gameplay\movement.rs --- 
use crate::components::*;
use crate::resources::controls::*;
use crate::utils::screen::PADDLE_SIZE;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const BALL_SPEED: f32 = 600.0;

pub fn move_paddle(
    time: Res<Time>,
    player_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut paddle_query: Query<(&mut Transform, &Paddle)>,
) {
    for (mut paddle_transform, paddle) in paddle_query.iter_mut() {
        if let Ok(action_state) = player_query.get(paddle.player) {
            let mut move_amount = time.delta_secs() * 400.0;
            
            if action_state.pressed(&PlayerAction::Speedup) {
                move_amount *= 2.0;
            }

            if action_state.pressed(&PlayerAction::Up){
                paddle_transform.translation.y += move_amount;
            }
            
            if action_state.pressed(&PlayerAction::Down) {
                paddle_transform.translation.y -= move_amount;
            }
        }
    }
}

pub fn maintain_ball_speed(
    mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
) {


    for mut velocity in ball_query.iter_mut() {
        let current_speed = velocity.length();
        if current_speed > 0.0 {
            velocity.0 = velocity.normalize() * BALL_SPEED;
        }
    }
}

pub fn paddle_hit_dynamics(
    mut collision_events: MessageReader<CollisionStart>,
    mut ball_query: Query<&mut LinearVelocity, With<Ball>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    ball_transform_query: Query<&Transform, With<Ball>>,
) {
    for (contacts) in collision_events.read() {
        let entity1 = contacts.collider1;
        let entity2 = contacts.collider2;

        // Determine which is ball and which is paddle
        let (ball_entity, paddle_entity) =
            if ball_query.contains(entity1) && paddle_query.contains(entity2) {
                (entity1, entity2)
            } else if ball_query.contains(entity2) && paddle_query.contains(entity1) {
                (entity2, entity1)
            } else {
                continue;
            };

        if let (Ok(mut ball_vel), Ok(paddle_transform), Ok(ball_transform)) =
            (ball_query.get_mut(ball_entity),
             paddle_query.get(paddle_entity),
             ball_transform_query.get(ball_entity)) {

            // Calculate hit offset from paddle center (-1.0 to 1.0)
            let paddle_half_height = PADDLE_SIZE.y/2.0;
            let offset = (ball_transform.translation.y - paddle_transform.translation.y)
                / paddle_half_height;
            
            let speed = ball_vel.length();
            let new_y_vel = offset * speed * 0.75;

            ball_vel.y = new_y_vel;
            // Maintain speed
            ball_vel.0 = ball_vel.normalize() * speed;
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\mod.rs --- 
pub mod gameplay;
pub mod ui;
pub mod settings;

pub use gameplay::*;
pub use ui::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\settings\mod.rs --- 
﻿pub mod persistence;
pub mod monitor; 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\settings\monitor.rs --- 
﻿use crate::resources::{BitDepth, MonitorInfo, Monitors, RefreshRate, Resolution};
use bevy::ecs::query::*;
use bevy::prelude::*;
use bevy::window::*;
use std::sync::Arc;

#[cfg(windows)]
fn get_monitor_name_windows(device_path: &str) -> Option<String> {
    use windows::Win32::Graphics::Gdi::*;
    use windows::core::PWSTR;

    let mut display_device = DISPLAY_DEVICEW::default();
    display_device.cb = size_of::<DISPLAY_DEVICEW>() as u32;

    let wide: Vec<u16> = device_path.encode_utf16().chain(Some(0)).collect();

    unsafe {
        if EnumDisplayDevicesW(
            PWSTR(wide.as_ptr() as *mut _),
            0,
            &mut display_device,
            0,
        )
            .as_bool()
        {
            let mut idx: usize = 0;

            for char in display_device.DeviceString {
                if char == 0{
                    break;
                }
                idx+=1;
            }

            return String::from_utf16(&display_device.DeviceString[..idx]).ok();
        }
    }

    None
}

pub fn on_spawn_monitors(
    query: Query<(Entity, &Monitor), Spawned>,
    window: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
){
    let mut info: Vec<MonitorInfo> = Vec::new();
    let mut current_monitor_index:usize = 0;
    let primary_window = window.single().expect("Couldn't get primary window");

    let selected_monitor = match primary_window.mode{
        WindowMode::BorderlessFullscreen(monitor) => Some(monitor),
        WindowMode::Fullscreen(monitor, _) => Some(monitor),
        _ => None,
    };

    for (index, (entity, monitor)) in query.iter().enumerate() {

        let selection = MonitorSelection::Entity(entity);
        let name = if let Some(real_name) = monitor.name.clone(){

            #[cfg(windows)]
            {
                if let Some(name) = get_monitor_name_windows(&real_name) {
                    name
                }else{
                    real_name
                }
            }
            #[cfg(not(windows))]
            {
                real_name
            }

        }else{
            format!("Monitor {}", index + 1)
        };


        if let Some(current_monitor) = selected_monitor {
            if current_monitor.eq(&selection){
                current_monitor_index = index;
            }
        }

        let mut refresh_rates: Vec<RefreshRate> = monitor.video_modes
            .iter()
            .map(|video_mode| video_mode.refresh_rate_millihertz.into())
            .collect();

        let mut resolutions: Vec<Resolution> = monitor.video_modes
            .iter()
            .map(|video_mode| video_mode.physical_size.into())
            .collect();

        let mut bit_depths: Vec<BitDepth> = monitor.video_modes
            .iter()
            .map(|video_mode| video_mode.bit_depth.into())
            .collect();

        resolutions.sort_unstable_by_key(|r| (r.0.x, r.0.y));
        resolutions.dedup();

        refresh_rates.sort_unstable();
        refresh_rates.dedup();

        bit_depths.sort_unstable();
        bit_depths.dedup();

        let bit_depth = bit_depths.iter().map(|x| x.0).max().expect("No bit depths");
        let refresh_rate = monitor.refresh_rate_millihertz.unwrap_or(
            refresh_rates.iter().map(|x| x.0).max().expect("No Refresh rates")
        );

        info.push(MonitorInfo{
                monitor_selection: selection,
                name,
                refresh_rates: Arc::new(refresh_rates),
                resolutions: Arc::new(resolutions),
                bit_depths: Arc::new(bit_depths),
                native_mode: VideoMode {
                    bit_depth,
                    refresh_rate_millihertz: refresh_rate,
                    physical_size: monitor.physical_size(),
                }
            });
    }

    commands.insert_resource(
        Monitors{
            monitors: Arc::new(info),
            selected_monitor: current_monitor_index,
        }
    );

} 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\settings\persistence.rs --- 
﻿use crate::resources::GameSettings;
use bevy::prelude::*;
use std::fs;

const SETTINGS_FILE: &str = "settings.json";

pub fn save_settings(settings: &Res<GameSettings>) {
    if settings.is_changed() {
        if let Ok(json) = serde_json::to_string_pretty(settings.as_ref()) {
            let _ = fs::write(SETTINGS_FILE, json);
        }
    }
}

pub fn load_settings() -> GameSettings {

    let settings: GameSettings;

    if let Ok(contents) = fs::read_to_string(SETTINGS_FILE) {
        if let Ok(loaded) = serde_json::from_str::<GameSettings>(&contents) {
            settings = loaded;
        }else{
            settings = GameSettings::default();
        }
    }else{
        settings = GameSettings::default();
    }

    settings
    
} 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\ui\menu.rs --- 
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

                    section.spawn(LabelBundle::button_label("Select Area Shape"));

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
// --- C:\Projects\PongEnhanced_4ITA\src\systems\ui\mod.rs --- 
pub mod selectors;
pub mod widgets;
pub mod menu; 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\ui\selectors.rs --- 
use crate::components::ui::{OptionSelector, SelectorText};
use bevy::prelude::*;

pub fn update_selector_text(
    selectors: Query<(Entity, &OptionSelector), Changed<OptionSelector>>,
    mut texts: Query<&mut Text, With<SelectorText>>,
    children: Query<&Children>,
) {
    for (selector_entity, selector) in &selectors {

        for child in children.iter_descendants(selector_entity) {
            if let Ok(mut text) = texts.get_mut(child) {
                text.0.clear();
                selector.push_current_string(&mut text.0);
                break;
            }
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\systems\ui\widgets.rs --- 
use crate::bundles::widgets::*;
use crate::components::ui::effects::{HoverLight, HoverLightColor};
use crate::components::ui::{Dropdown, OptionSelector, SelectorButton, SelectorText, SourceHandle, UIOptionProvider};
use crate::events::widgets::{ButtonPressed, OptionChanged};
use crate::utils::{lighten_color, DEFAULT_LIGHTEN_AMOUNT, MODERN_THEME};
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy::ui_widgets::{Slider, SliderPrecision, SliderRange, SliderThumb, SliderValue, TrackClick};
use bevy_tween::combinator::AnimationBuilderExt;
use bevy_tween::interpolate::background_color_to;
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::IntoTarget;
use std::sync::Arc;
use std::time::Duration;

pub const BUTTON_PADDING: Val = Val::Px(20.0);
pub const PIXEL_BORDER: UiRect = UiRect::all(Val::Px(3.0)); // Classic pixel border width
pub const BUTTON_OUTLINE: Outline = Outline::new(PIXEL_BORDER.bottom, Val::ZERO, Color::BLACK);


pub fn u_slider_visuals(
    sliders: Query<(Entity, &SliderValue, &SliderRange), Changed<SliderValue>>,
    children: Query<&Children>,
    mut thumbs: Query<&mut Node, With<SliderThumb>>,
) {
    for (slider_entity, value, range) in sliders.iter() {
        for child in children.iter_descendants(slider_entity) {
            if let Ok(mut thumb_node) = thumbs.get_mut(child) {
                thumb_node.left = percent(range.thumb_position(value.0) * 100.0);
            }
        }
    }
}

pub fn u_ui_hover_light(
    mut commands: Commands,
    query: Query<(Entity, Ref<Interaction>, &HoverLight, Option<&HoverLightColor>),
    (Changed<Interaction>, With<BackgroundColor>)>,
) {
    for (entity, interaction, hover_light, maybe_custom_colors) in &query {
        let base_color = hover_light.0;

        if interaction.is_added() || !interaction.is_changed() {
            continue;
        }

        let hover_color = if let Some(custom) = maybe_custom_colors {
            custom.hover_color
        } else {
            lighten_color(base_color, DEFAULT_LIGHTEN_AMOUNT)
        };


        let mut target = entity.into_target();

        match *interaction {
            Interaction::Hovered => {


                commands.entity(entity).animation().insert_tween_here(
                    Duration::from_millis(250),
                    EaseKind::CubicInOut,
                    target.state(base_color).with(background_color_to(hover_color))
                );
            },
            Interaction::None => {

                commands.entity(entity).animation().insert_tween_here(
                    Duration::from_millis(250),
                    EaseKind::CubicInOut,
                    target.state(hover_color).with(background_color_to(base_color))
                );
            }
            _ => {}
        };
    }
}

pub fn w_button(color: Color, size: Vec2, text: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(BUTTON_PADDING),
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(color),
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        Outline::new(PIXEL_BORDER.bottom, Val::ZERO, MODERN_THEME.outline),
        HoverLight(color),
        Children::spawn_one(LabelBundle::button_label(text)),
    )
}

pub fn w_menu_title(text: &'static str) -> impl Bundle {
    w_title(text, 72.0)
}

pub fn w_title(text: &'static str, size: f32) -> impl Bundle {
    (
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            padding: UiRect::all(Val::Px(10.0)),
            border: PIXEL_BORDER,
            ..default()
        },
        BorderColor::from(MODERN_THEME.accent),
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(MODERN_THEME.text_bright),
    )
}

pub fn w_slider(min: f32, max: f32, current: f32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(50),
            width: percent(100),
            ..default()
        },
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderPrecision(0),
        SliderValue(current),
        SliderRange::new(min, max),
        Children::spawn((
            Spawn((
                Node {
                    height: px(12),
                    border: PIXEL_BORDER,
                    ..default()
                },
                BackgroundColor(MODERN_THEME.slider_track),
                BorderColor::from(MODERN_THEME.border),
                BorderRadius::ZERO,
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(20),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                Children::spawn_one(w_slider_thumb(Vec2::new(20.0,20.0)))
            )),
        )),
    )
}

pub fn w_menu_section() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(30.0)),
            margin: UiRect::all(Val::Px(10.0)),
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(MODERN_THEME.section_bg),
        BorderColor::from(MODERN_THEME.border_dark),
        BorderRadius::ZERO,
    )
}


pub fn w_slider_thumb(size: Vec2) -> impl Bundle {
    (
        SliderThumb,
        Node {
            display: Display::Flex,
            width: px(size.x),
            height: px(size.y),
            position_type: PositionType::Absolute,
            left: percent(0),
            border: PIXEL_BORDER,
            ..default()
        },
        BorderRadius::ZERO,
        BorderColor::from(MODERN_THEME.border),
        BackgroundColor(MODERN_THEME.slider_thumb),
    )
}

pub fn w_dropdown(options: Arc<dyn UIOptionProvider>, selected: usize, tab_index: i32) -> impl Bundle {
    (
        Dropdown {
            selected,
            options,
        },
        Node {
            width: Val::Px(300.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(MODERN_THEME.panel_bg),
        BorderColor::from(MODERN_THEME.border),
        BorderRadius::ZERO,
        HoverLight(MODERN_THEME.panel_bg),
        TabIndex(tab_index),
    )
}
pub fn w_selector(options_provider: SourceHandle<dyn UIOptionProvider>, selected: usize, label: impl Into<String>) -> impl Bundle {
    (
        OptionSelector {
            options_provider,
            selected
        },
        Node {
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(20.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        Children::spawn((
            Spawn((
                w_button(MODERN_THEME.button, Vec2::new(40.0, 40.0), "<"),
                SelectorButton(false),
            )),
            Spawn((
                Node {
                    width: Val::Px(450.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(15.0)),
                    border: PIXEL_BORDER,
                    ..default()
                },
                BackgroundColor(MODERN_THEME.panel_bg),
                BorderColor::from(MODERN_THEME.border),
                BorderRadius::ZERO,
                Children::spawn_one((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    Children::spawn((
                        Spawn(LabelBundle::button_label(label)),
                        Spawn((
                            TextFont {
                                font_size: 32.0,
                                font_smoothing: FontSmoothing::None,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            SelectorText,
                        )),
                    )),
                ))
            )),
            Spawn((
                w_button(MODERN_THEME.button, Vec2::new(40.0, 40.0), ">"),
                SelectorButton(true),
            ))
        ))
    )
}

pub fn w_section_header(text: &'static str) -> impl Bundle {
    (
        Node {
            margin: UiRect::new(Val::ZERO, Val::ZERO, Val::Px(20.0), Val::Px(10.0)),
            ..default()
        },
        Text::new(text),
        TextFont {
            font_size: 36.0,
            ..default()
        },
        TextColor(MODERN_THEME.accent),
    )
}

pub fn w_labeled_slider(min: f32, max: f32, current: f32, label: &str) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(10.0),
            ..default()
        },
        Children::spawn((
            Spawn(LabelBundle::button_label(label)),
            Spawn(w_slider(min, max, current)),
        )),
    )
}

pub fn w_menu_button(color: Color, text: &str) -> impl Bundle {
    w_button(color, Vec2::new(350.0, 70.0), text)
}

pub fn update_selector(
    pressed: On<ButtonPressed>,
    mut selectors: Query<&mut OptionSelector>,
    button: Query<(&ChildOf, &SelectorButton)>,
    mut commands: Commands)
{

    if let Ok((child_of, button)) = button.get(pressed.event_target()) {
        let selector_entity = child_of.parent();
        if let Ok(mut selector) = selectors.get_mut(selector_entity) {

        if button.0 {
            selector.next();
        } else {
            selector.prev();
        }

        commands.trigger(OptionChanged(selector_entity));
        }
    }
}

pub fn w_row_container(gap: f32) -> impl Bundle {
    (
        Node{
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            row_gap: Val::Px(gap),
            ..default()
        }
    )
}

pub fn w_container(size: Vec2) -> impl Bundle {
    (
        Node{
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            border: PIXEL_BORDER,
            ..default()
        },
        BackgroundColor(MODERN_THEME.section_bg),
        BorderColor::from(MODERN_THEME.border),
    )
}

pub fn t_button_press(
    button_query: Query<Entity, (With<Button>, With<Interaction>)>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for entity in &button_query {
        if let Ok(interaction) = interaction_query.get(entity) {
            if *interaction == Interaction::Pressed {
                commands.trigger(ButtonPressed(entity));
            }
        }
    }
} 
// --- C:\Projects\PongEnhanced_4ITA\src\traits\game\mod.rs --- 
pub mod settings; 
// --- C:\Projects\PongEnhanced_4ITA\src\traits\game\settings.rs --- 
use bevy::prelude::*;
use std::any::TypeId;

pub trait SelectableResource: Send + Sync + 'static {
    fn get_options() -> Vec<(&'static str, Self)> where Self: Sized;
    fn get_label(&self) -> &'static str;
} 
// --- C:\Projects\PongEnhanced_4ITA\src\traits\mod.rs --- 
pub mod game; 
// --- C:\Projects\PongEnhanced_4ITA\src\utils\mod.rs --- 
pub mod screen;
pub mod text;

pub use screen::*;
pub use text::*; 
// --- C:\Projects\PongEnhanced_4ITA\src\utils\screen.rs --- 
use bevy::prelude::*;
use avian2d::prelude::LinearDamping;

pub const FIXED_DIMENSIONS: Vec2 = Vec2::new(1280.0, 720.0);
pub const ZERO_DAMPING: LinearDamping = LinearDamping(0.0);
pub const PADDLE_SIZE: Vec2 = Vec2::new(12.5, 100.0);
pub const BALL_RADIUS: f32 = 7.0;
pub const WALL_THICKNESS: f32 = 0.0;
pub const HALF_WIDTH: f32 = FIXED_DIMENSIONS.x / 2.0;
pub const HALF_HEIGHT: f32 = FIXED_DIMENSIONS.y / 2.0;
pub const HALF_WALL_THICKNESS: f32 = WALL_THICKNESS / 2.0;

pub const PADDLE_WALL_PADDING: f32 = 25.0;
pub const TRANSFORM_ZERO: Transform = Transform::from_xyz(0.0,0.0,0.0); 
// --- C:\Projects\PongEnhanced_4ITA\src\utils\text.rs --- 
use bevy::prelude::*;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../../assets/font/jersey10_regular.ttf");
pub const DEFAULT_LIGHTEN_AMOUNT: f32 = 30.0;
pub const PIXEL_BORDER_SIZE: f32 = 3.0;


#[derive(Clone, Copy)]
pub struct RetroTheme {
    pub button: Color,
    pub button_hover: Color,
    pub button_pressed: Color,
    pub slider_track: Color,
    pub slider_thumb: Color,
    pub border: Color,
    pub border_dark: Color,
    pub outline: Color,
    pub accent: Color,
    pub panel_bg: Color,
    pub section_bg: Color,
    pub text_bright: Color,
    pub text_normal: Color,
}

pub const MODERN_THEME: RetroTheme = RetroTheme {
    button: Color::srgb(0.15, 0.15, 0.15),
    button_hover: Color::srgb(0.25, 0.25, 0.25),
    button_pressed: Color::srgb(0.35, 0.75, 0.35),
    slider_track: Color::srgb(0.05, 0.05, 0.05),
    slider_thumb: Color::srgb(0.35, 0.75, 0.35),
    border: Color::srgb(0.45, 0.45, 0.45),
    border_dark: Color::srgb(0.3, 0.3, 0.3),
    outline: Color::srgb(0.45, 0.45, 0.45),
    accent: Color::srgb(0.35, 0.75, 0.35),
    panel_bg: Color::srgb(0.2, 0.2, 0.25),
    section_bg: Color::srgb(0.1, 0.1, 0.1),
    text_bright: Color::srgb(0.9, 0.9, 1.0),
    text_normal: Color::srgb(0.8, 0.8, 0.8),
};

pub fn lighten_color(color: Color, amount: f32) -> Color {
    let [r, g, b, a] = color.to_srgba().to_f32_array();
    Color::srgba(
        (r + amount / 100.0).min(1.0),
        (g + amount / 100.0).min(1.0),
        (b + amount / 100.0).min(1.0),
        a,
    )
} 
