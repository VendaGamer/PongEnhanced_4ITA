use bevy::prelude::Component;
use crate::components::team::Team;

#[derive(Component)]
pub enum Area{
    Cuboid{
        left_team: Team,
        right_team: Team,
        top_team: Team,
        low_team: Team
    },
    Triangular{
        left_team: Team,
        right_team: Team,
        low_team: Team
    },
    TwoSide{
        left_team: Team,
        right_team: Team
    },
}
