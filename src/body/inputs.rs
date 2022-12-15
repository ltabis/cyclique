use crate::simulation::orbit_visualizer::lines::LineMaterial;
use bevy::prelude::*;

use super::new_body;

pub fn body_inputs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        new_body(
            Vec3::ZERO,
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut line_materials,
        )
    }
}
