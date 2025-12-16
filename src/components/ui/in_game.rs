use crate::bundles::Component;
use crate::models::game::area::AreaSide;

#[derive(Component)]
pub struct ScoreText {
    pub area_side: AreaSide,
}