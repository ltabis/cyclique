use crate::simulation::orbit_visualizer::lines::LineMaterial;
use bevy::prelude::*;

use super::new_body;

pub fn body_inputs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        new_body(
            Vec3::ZERO,
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut line_materials,
        )
    }
}
