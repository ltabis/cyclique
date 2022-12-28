use crate::simulation::orbit_visualizer::lines::LineMaterial;
use bevy::prelude::*;

use super::new_body;

pub fn body_inputs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    cam: Query<&Transform, With<Camera3d>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::N) {
        let mut transform = cam.iter().next().expect("main camera not found").clone();
        let mut forward = transform.forward();
        forward.z += 10.;
        let translation = transform.rotation * forward;
        transform.translation += translation;

        new_body(
            transform,
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut line_materials,
        );
    }
}
