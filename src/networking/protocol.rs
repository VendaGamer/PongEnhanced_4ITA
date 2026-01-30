use avian2d::prelude::Position;
use bevy::prelude::*;
use lightyear::prelude::{AppComponentExt, InterpolationRegistrationExt, PeerId, PredictionRegistrationExt};
use serde::*;
use crate::bundles::App;
use crate::models::game::area::LocalPlayerID;

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PaddlePosition(pub Vec2);
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerPosition(pub Vec2);


#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
pub struct PlayerId(pub PeerId, pub LocalPlayerID);

pub struct GameProtocolPlugin;

impl Plugin for GameProtocolPlugin {
    fn build(&self, app: &mut App) {

        app.register_component::<PlayerPosition>()
            .add_prediction()
            .add_linear_interpolation();


        app.register_component::<LinearVelocity>().add_prediction();

        app.register_component::<AngularVelocity>().add_prediction();
    }
}