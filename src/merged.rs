// --- ./bundles/area.rs ---
use bevy::prelude::*;
use crate::components::area::Area;
use crate::components::AreaShape;
use crate::bundles::GoalBundle;
use crate::bundles::wall::WallBundle;
use crate::components::side::Side;
use crate::utils::screen::TRANSFORM_ZERO;

#[derive(Bundle, Clone, Copy)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle {
    pub fn spawn(
        area_shape: AreaShape,
        commands: &mut Commands,
        teams: &[Entity],
    ) -> Entity {


        match area_shape {
            AreaShape::TwoSide => {
                if teams.len() >= 2 {
                    commands.spawn(GoalBundle::new(teams[0], Side::Left));
                    commands.spawn(GoalBundle::new(teams[1], Side::Right));
                    commands.spawn(WallBundle::new(Side::Bottom));
                    commands.spawn(WallBundle::new(Side::Top));

                }
            }
            AreaShape::Triangular => {
                if teams.len() >= 3 {
                    commands.spawn(GoalBundle::new(teams[0], Side::Left));
                    commands.spawn(GoalBundle::new(teams[1], Side::Right));
                    commands.spawn(GoalBundle::new(teams[2], Side::Bottom));
                    commands.spawn(WallBundle::new(Side::Top));
                }
            }
            AreaShape::Cuboid => {
                if teams.len() >= 4 {
                    commands.spawn(GoalBundle::new(teams[0], Side::Left));
                    commands.spawn(GoalBundle::new(teams[1], Side::Right));
                    commands.spawn(GoalBundle::new(teams[2], Side::Bottom));
                    commands.spawn(GoalBundle::new(teams[3], Side::Top));
                }
            }
        }

        commands.spawn(AreaBundle{
            transform: TRANSFORM_ZERO,
            area: Area{
                shape: area_shape
            },
            global_transform: GlobalTransform::from(TRANSFORM_ZERO)
        }).id()
    }
}

// --- ./bundles/ball.rs ---
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

// --- ./bundles/camera.rs ---
use bevy::camera;
use bevy::color::Color;
use bevy::prelude::*;
use crate::ScalingMode;
use crate::utils::FIXED_DIMENSIONS;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera2d: Camera2d,
    pub camera: Camera,
    pub projection: Projection,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            camera2d: Camera2d,
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            projection: Projection::from(OrthographicProjection {
                scaling_mode: camera::ScalingMode::Fixed {
                    width: FIXED_DIMENSIONS.x,
                    height: FIXED_DIMENSIONS.y,
                },
                ..OrthographicProjection::default_2d()
            }),
        }
    }
}

// --- ./bundles/division_line.rs ---
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
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
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

// --- ./bundles/goal.rs ---
use crate::components::side::Side;
use crate::components::Goal;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Bundle, Entity, Transform};

#[derive(Bundle)]
pub struct GoalBundle{
    pub goal: Goal,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl GoalBundle{
    pub fn new(team: Entity, side: Side) -> Self{
        let collider = Side::get_collider(side.clone());
        let transform = Side::get_transform(side.clone());

        Self{
            goal: Goal{
                team,
                side
            },
            collider,
            transform,
            rigid_body: RigidBody::Static
        }
    }
}

// --- ./bundles/mod.rs ---
pub mod paddle;
pub mod ball;
pub mod camera;
pub mod area;
pub mod player;
pub mod goal;
pub mod wall;
pub mod division_line;

pub use ball::*;
pub use paddle::*;
pub use camera::*;
pub use bevy::prelude::*;
pub use goal::*;
pub use division_line::*;

// --- ./bundles/paddle.rs ---
use crate::Paddle;
use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::Player;

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
        player: Entity
    ) -> Self {
        Self {
            paddle: Paddle{
                player
            },
            mesh: Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(size.x, size.y),
            restitution: Restitution::new(1.0),
        }
    }
}

// --- ./bundles/player.rs ---
use crate::bundles::Entity;
use crate::components::{Paddle, Player};
use crate::resources::controls::PlayerAction;
use bevy::prelude::Bundle;
use leafwing_input_manager::input_map::InputMap;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub bindings: InputMap<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(player: Player) -> Self{
        let bindings = Player::get_default_input_map(&player);
        PlayerBundle{
            player,
            bindings
        }
    }
}

// --- ./bundles/wall.rs ---
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::Transform;
use crate::bundles::Bundle;
use crate::components::side::Side;
use crate::components::wall::Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl WallBundle {
    pub fn new(side: Side) -> Self {
        let collider = Side::get_collider(side);
        let transform = Side::get_transform(side);

        Self {
            wall: Wall {
                side
            },
            collider,
            transform,
            rigid_body: RigidBody::Static
        }
    }
}

// --- ./components/area.rs ---
use crate::components::area_shape::AreaShape;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Area{
    pub shape: AreaShape
}

// --- ./components/area_shape.rs ---
#[derive(Clone, Copy)]
pub enum AreaShape {
    TwoSide,
    Triangular,
    Cuboid,
}

// --- ./components/ball.rs ---
use bevy::prelude::Component;
use crate::bundles::Vec2;

#[derive(Component)]
pub struct Ball{
    pub initial_velocity: Vec2
}

// --- ./components/division_line.rs ---
use bevy::prelude::Component;

#[derive(Component)]
pub struct DivisionLine;

// --- ./components/game_mode.rs ---
use bevy::app::App;

#[derive(Clone, Copy)]
pub enum GameMode {
    Classic,
    UpsideDown,
    Modern,
    BlackOut,
    Twisted,
}

pub trait GameModeRules: Send + Sync {
    fn ball_speed(&self) -> f32;
    fn gravity_scale(&self) -> f32;
    fn paddle_speed_multiplier(&self) -> f32;
    fn apply_special_mechanics(&self, app: &mut App);
}

// --- ./components/goal.rs ---
use bevy::prelude::{Component, Entity};
use crate::components::side::Side;

#[derive(Component)]
pub struct Goal {
    pub team: Entity,
    pub side: Side
}

// --- ./components/mod.rs ---
pub mod paddle;
pub mod ball;
pub mod team;
pub mod player;
pub mod area;
pub mod side;
pub mod goal;
pub mod wall;
pub mod area_shape;
pub mod game_mode;
pub mod score_text;
pub mod division_line;


pub use division_line::*;
pub use ball::*;
pub use paddle::*;
pub use player::*;
pub use team::*;
pub use goal::*;
pub use area_shape::*;
pub use area::*;
pub use game_mode::*;
pub use score_text::*;

// --- ./components/paddle.rs ---
use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Paddle{
    pub player: Entity
}

// --- ./components/player.rs ---
use crate::resources::controls::PlayerAction;
use bevy::prelude::{Component, Entity, KeyCode};
use leafwing_input_manager::prelude::InputMap;

#[derive(Component)]
pub struct Player {
    pub id: u8,
    pub team: Entity,
    pub name: String,
}

impl Player{
    pub fn get_default_input_map(player : &Self) -> InputMap<PlayerAction> {
        match player.id {
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

// --- ./components/score_text.rs ---
use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct ScoreText {
    pub team: Entity,
}

// --- ./components/side.rs ---
use avian2d::prelude::Collider;
use bevy::prelude::{default, Component, Transform, Vec3};
use crate::utils::FIXED_DIMENSIONS;
use crate::utils::screen::*;

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}


impl Side {
    pub fn get_transform(side: Self) -> Transform {


        match side {
            Side::Left => Transform {
                translation: Vec3::new(-HALF_WIDTH - HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            Side::Right => Transform {
                translation: Vec3::new(HALF_WIDTH + HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            Side::Top => Transform {
                translation: Vec3::new(0.0, HALF_HEIGHT + HALF_WALL_THICKNESS, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
            Side::Bottom => Transform {
                translation: Vec3::new(0.0, -HALF_HEIGHT - HALF_WALL_THICKNESS / 2.0, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
        }
    }


    pub fn get_collider(side: Self) -> Collider {
        match side {
            Side::Left | Side::Right => {
                Collider::rectangle(WALL_THICKNESS, FIXED_DIMENSIONS.y)
            }
            Side::Top | Side::Bottom => {
                Collider::rectangle(FIXED_DIMENSIONS.x, WALL_THICKNESS)
            }
        }
    }

}


// --- ./components/team.rs ---
use bevy::prelude::Component;

#[derive(Component)]
pub struct Team {
    pub name: String,
    pub current_score: u32
}

// --- ./components/wall.rs ---
use bevy::prelude::Component;
use crate::components::side::Side;

#[derive(Component)]
pub struct Wall {
    pub side: Side,
}

// --- ./events/mod.rs ---


// --- ./main.rs ---
mod components;
mod resources;
mod systems;
mod bundles;
mod plugins;
mod utils;
mod events;

use crate::plugins::GameCorePlugin;
use crate::resources::controls::PlayerAction;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::window::PresentMode;
use avian2d::prelude::*;
use components::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pong Enhanced".into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::all()),
                    ..default()
                }),
                ..default()
            }),

            PhysicsPlugins::default(),
            GameCorePlugin,
            FpsOverlayPlugin{
                config: FpsOverlayConfig{
                    enabled: true,
                    text_color: Srgba::rgb(1.0, 0.73, 0.23).into(),
                    frame_time_graph_config: FrameTimeGraphConfig{
                        enabled: false,
                        ..default()
                    },
                    ..default()
                },
            },
            InputManagerPlugin::<PlayerAction>::default(),
        ))
        .run();
}


// --- ./plugins/game_core.rs ---
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::bundles::area::AreaBundle;
use crate::components::*;
use crate::systems::*;
use crate::utils::screen::{BALL_RADIUS, HALF_WIDTH, PADDLE_SIZE, PADDLE_WALL_PADDING};

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
            (
                move_paddle,
                check_connection,
                maintain_ball_speed,
                paddle_hit_dynamics,
                update_score_ui
            ))
            .add_systems(Startup, (
                setup,
                setup_ui
            ));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let team1 = commands.spawn(Team {
        name: "Team Left".into(),
        current_score: 0,
    }).id();

    let team2 = commands.spawn(Team {
        name: "Team Right".into(),
        current_score: 0,
    }).id();

    let player1=commands.spawn(PlayerBundle::new(
        Player {
            id: 1,
            team: team1,
            name: "Player 1".into(),
        }
    )).id();

    let player2 = commands.spawn(PlayerBundle::new(
        Player {
            id: 2,
            team: team2,
            name: "Player 2".into(),
        }
    )).id();

    let paddle1 = commands.spawn(
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(-HALF_WIDTH + PADDLE_WALL_PADDING, 0.0, 0.0),
            PADDLE_SIZE,
            player1
        )
    ).id();

    let paddle2 = commands.spawn(
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(HALF_WIDTH - PADDLE_WALL_PADDING, 0.0, 0.0),
            PADDLE_SIZE,
            player2
        )
    ).id();


    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-300.0, 300.0),
        BALL_RADIUS
    )).observe(handle_scoring);

    commands.spawn(CameraBundle::default());

    let teams = [team1, team2];
    AreaBundle::spawn(AreaShape::TwoSide, &mut commands, &teams);
    commands.spawn(DivisionLineBundle::new(meshes, materials));
}


fn setup_ui(
    mut commands: Commands,
    teams: Query<Entity, With<Team>>,
) {

    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Start,
        padding: UiRect::all(Val::Px(50.0)),
        ..default()
    }).with_children(|parent| {
        let team_entities: Vec<Entity> = teams.iter().collect();

        if team_entities.len() >= 2 {
            // Left team score
            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ScoreText { team: team_entities[0] },
            ));


            parent.spawn((
                Text::new("0"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ScoreText { team: team_entities[1] },
            ));
        }
    });
}

fn update_score_ui(
    teams: Query<&Team>,
    mut score_texts: Query<(&mut Text, &ScoreText)>,
) {
    for (mut text, score_text) in score_texts.iter_mut() {
        if let Ok(team) = teams.get(score_text.team) {
            text.0 = team.current_score.to_string();
        }
    }
}

// --- ./plugins/mod.rs ---
pub mod game_core;
mod state_persistence;

pub use game_core::*;



// --- ./plugins/state_persistence.rs ---
use bevy::app::App;
use bevy::prelude::*;

pub struct GameStatePersistencePlugin;

impl Plugin for GameStatePersistencePlugin{
    fn build(&self, app: &mut App) {
    app.add_plugins(AssetPlugin::default());
    }
}

// --- ./resources/controls.rs ---
use bevy::reflect::Reflect;
use leafwing_input_manager::Actionlike;

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

// --- ./resources/game_config.rs ---
use bevy::prelude::Resource;
use crate::components::area::Area;
use crate::components::{AreaShape, GameMode};

#[derive(Resource)]
pub struct GameModeConfig {
    pub current_mode: GameMode,
    pub initial_ball_speed: f32,
}

impl GameModeConfig {
    pub fn get_ball_speed(&self) -> f32 {
        match self.current_mode {
            GameMode::Classic => 400.0,
            GameMode::Modern => 600.0,
            GameMode::Twisted => 450.0,
            _ => 400.0,
        }
    }

    pub fn get_paddle_speed(&self) -> f32 {
        match self.current_mode {
            GameMode::Classic => 400.0,
            GameMode::UpsideDown => 300.0,
            GameMode::Modern => 500.0,
            _ => 400.0,
        }
    }
}

// --- ./resources/mod.rs ---
pub mod controls;
mod game_config;



// --- ./systems/handle_gamepads.rs ---
use bevy::input::gamepad::GamepadConnectionEvent;
use bevy::prelude::{Color, MessageReader};

pub fn check_connection(mut events: MessageReader<GamepadConnectionEvent>){
    for ev in events.read(){
        
    }
}

// --- ./systems/handle_scoring.rs ---
use bevy::prelude::*;
use avian2d::prelude::*;
use crate::bundles::BallBundle;
use crate::components::*;
use crate::utils::screen::BALL_RADIUS;

pub fn handle_scoring(
    collision: On<CollisionStart>,
    goals: Query<&Goal>,
    mut teams: Query<&mut Team>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let ball = collision.collider1;
    let other = collision.collider2;

    if let Ok(goal) = goals.get(other){

        if let Ok(mut team) = teams.get_mut(goal.team) {
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

// --- ./systems/mod.rs ---
pub mod movement;
pub mod handle_gamepads;
pub mod handle_scoring;

pub use movement::*;
pub use handle_scoring::*;
pub use handle_gamepads::*;

// --- ./systems/movement.rs ---
use bevy::prelude::*;
use avian2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::components::*;
use crate::resources::controls::*;

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

            if action_state.pressed(&PlayerAction::Up) {
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
    const BALL_SPEED: f32 = 400.0;

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
            let paddle_half_height = 100.0;
            let offset = (ball_transform.translation.y - paddle_transform.translation.y)
                / paddle_half_height;

            // Influence vertical velocity based on hit position
            let speed = ball_vel.length();
            let new_y_vel = offset * speed * 0.75;

            ball_vel.y = new_y_vel;
            // Maintain speed
            ball_vel.0 = ball_vel.normalize() * speed;
        }
    }
}

// --- ./utils/mod.rs ---
pub mod screen;

pub use screen::FIXED_DIMENSIONS;

// --- ./utils/screen.rs ---
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

