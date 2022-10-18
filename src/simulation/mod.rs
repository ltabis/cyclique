pub mod math;
pub mod orbit_visualizer;

use bevy::prelude::*;

use crate::body::components::{Body, Velocity};

use self::orbit_visualizer::lines::{LineMaterial, LineStrip};

pub fn display_orbits(
    mut visualize: ResMut<orbit_visualizer::OrbitVisualizer>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,

    bodies: Query<Entity, With<Body>>,
    q_body: Query<(&Body, &Transform, &mut Velocity)>,
) {
    for body in &bodies {
        if let Some(points) = visualize.as_mut().simulate_orbits(&body, &bodies, &q_body) {
            // Render the orbit using line strips.
            commands.spawn().insert_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(LineStrip { points })),
                transform: Transform::from_xyz(0.5, 0.0, 0.0),
                material: materials.add(LineMaterial { color: Color::RED }),
                ..default()
            });
        }
    }
}
