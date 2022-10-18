pub mod lines;

use bevy::prelude::*;

use crate::body::components::{Body, Velocity};

use super::math::compute_acceleration;

/// A resource used to simulate and display orbits of bodies.
pub struct OrbitVisualizer {
    /// Number of steps to compute to predict the orbit of an object.
    iterations: usize,
    /// As the simulation been completed.
    done: std::collections::HashSet<Entity>,
}

impl Default for OrbitVisualizer {
    fn default() -> Self {
        Self::new(20)
    }
}

impl OrbitVisualizer {
    /// Create a new visualizer.
    ///
    /// # Args
    /// * `iterations` - number of iterations to compute when calling [`Self::simulate_orbits`].
    pub fn new(iterations: usize) -> Self {
        Self {
            iterations,
            done: std::collections::HashSet::new(),
        }
    }

    /// Simulate bodies orbits using [`Self::iterations`].
    ///
    /// Each simulation creates [`Self::iterations`] lines.
    pub fn simulate_orbits(
        &mut self,
        entity: &Entity,
        others: &Query<Entity, With<Body>>,
        q_body: &Query<(&Body, &Transform, &mut Velocity)>,
    ) -> Option<Vec<Vec3>> {
        if self.done.contains(entity) {
            return None;
        }

        let (_, transform, velocity) = q_body
            .get(*entity)
            .expect("body/transform components are missing from a body");

        let mut transform = transform.clone();
        let mut velocity = velocity.clone();
        let mut points = Vec::with_capacity(self.iterations);

        // Initial position.
        points.push(transform.translation);

        // Simulate the desired body's trajectory.
        for _ in 0..self.iterations {
            let acceleration = compute_acceleration(entity, others, q_body);

            velocity.0 += acceleration;
            transform.translation += velocity.0;

            points.push(transform.translation);
        }

        self.done.insert(entity.clone());

        Some(points)
    }
}
