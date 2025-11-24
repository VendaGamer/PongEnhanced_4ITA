use crate::bundles::{default, Bundle, FlexDirection, Node, UiRect, Val};

#[derive(Bundle)]
pub struct Container {
    container: Node,
}

impl Container {
    pub fn main_menu() -> Self {
        Self {
            container: Node {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }
        }
    }
    pub fn buttons() -> Self {
        Self {
            container: Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            }
        }
    }
}