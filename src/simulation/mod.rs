pub mod math;
pub mod orbit_visualizer;

use bevy::prelude::*;

use crate::body::components::{Body, Velocity};

use self::orbit_visualizer::lines::LineMaterial;

pub fn call_orbit_visualizer(
    mut visualize: ResMut<orbit_visualizer::OrbitVisualizer>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,

    bodies: Query<Entity, With<Body>>,
    q_body: Query<(&Body, &Transform, &Velocity)>,
) {
    for body in &bodies {
        visualize.as_mut().simulate_orbits(
            &mut commands,
            &mut meshes,
            &mut materials,
            &body,
            &bodies,
            &q_body,
        )
    }
}
