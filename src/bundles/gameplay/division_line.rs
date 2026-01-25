use crate::components::DivisionLine;
use bevy::prelude::*;

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