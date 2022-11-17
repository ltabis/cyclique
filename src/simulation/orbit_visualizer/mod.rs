pub mod lines;
pub mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use self::lines::{LineMaterial, LineStrip};

/// Contain orbit metadata with lines to debug a body's orbit.
#[derive(Component, Inspectable)]
pub struct OrbitVisualizer {
    /// Number of steps to compute to predict the orbit of an object.
    pub iterations: usize,
    /// Entity marked with the `OrbitLines` component.
    pub lines: Entity,
}

impl OrbitVisualizer {
    /// Create a new visualizer.
    ///
    /// # Args
    /// * `iterations` - number of iterations to compute when calling [`Self::simulate_orbits`].
    pub fn new(
        iterations: usize,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<LineMaterial>>,
    ) -> Self {
        Self {
            iterations,
            lines: commands
                .spawn()
                .insert_bundle(MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(LineStrip { points: vec![] })),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    material: materials.add(LineMaterial { color: Color::RED }),
                    ..default()
                })
                .insert(OrbitLines)
                .id(),
        }
    }
}

/// Component associated with a line mesh to draw an orbit.
#[derive(Component)]
pub struct OrbitLines;
