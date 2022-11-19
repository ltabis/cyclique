use bevy::prelude::*;

use crate::{
    body::components::{Body, Velocity},
    simulation::{math::compute_acceleration, orbit_visualizer::lines::LineStrip},
};

use super::{OrbitLines, OrbitVisualizer};

/// Simulate bodies orbits for [`Self::iterations`] iterations.
///
/// Each simulation creates [`Self::iterations`] lines.
pub fn simulate_orbits(
    state: Res<crate::body::state::State>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut orbits: Query<(Entity, &mut OrbitVisualizer)>,
    lines: Query<(Entity, &mut Handle<Mesh>), With<OrbitLines>>,
    q_body: Query<(Entity, &Body, &Transform, &Velocity)>,
) {
    if !state.paused {
        return;
    }

    let max_iterations = orbits.iter().fold(0, |acc, (_, orbit)| {
        if orbit.iterations > acc {
            orbit.iterations
        } else {
            acc
        }
    });

    // FIXME: Constant allocs, to refactor.
    let mut tmp: std::collections::HashMap<Entity, (_, _, _)> = q_body
        .iter()
        .map(|(entity, body, transform, velocity)| {
            (entity, (body, transform.clone(), velocity.clone()))
        })
        .collect();

    let mut new_lines: std::collections::HashMap<Entity, Vec<_>> = orbits
        .iter()
        .map(|(entity, orbit)| {
            let (line_entity, _) = lines.get(orbit.lines).unwrap();
            let (_, _, transform, _) = q_body.get(entity).unwrap();

            // Adding the initial point to the list of orbit lines to render.
            (line_entity, vec![transform.translation])
        })
        .collect();

    for iteration in 0..max_iterations {
        // FIXME: move this one block up & clear each iteration to reduce allocations.
        let mut accelerations = Vec::with_capacity(tmp.len());

        {
            let tmp = &tmp;

            for (entity, (body, transform, _)) in tmp {
                if body.fixed {
                    continue;
                }

                let mut acceleration = Vec3::default();

                for (other, (o_body, o_transform, _)) in tmp {
                    if entity == other {
                        continue;
                    }

                    acceleration += compute_acceleration(body, transform, o_body, o_transform);
                }

                accelerations.push((*entity, acceleration));
            }
        }

        // Update orbits with current velocities.
        for (entity, acceleration) in accelerations {
            let (_, transform, velocity) = tmp.get_mut(&entity).unwrap();

            velocity.0 += acceleration;
            transform.translation += velocity.0;

            if let Ok((_, orbit)) = orbits.get_mut(entity) {
                if iteration < orbit.iterations {
                    new_lines
                        .get_mut(&orbit.lines)
                        .expect("every orbit visualizer should have a valid line mesh child")
                        .push(transform.translation);
                }
            }
        }
    }

    for (_, orbit) in &orbits {
        // Update line strips positions.
        let points = new_lines.get(&orbit.lines).unwrap().clone();
        let (_, mesh) = lines.get(orbit.lines).unwrap();
        let mesh = meshes.get_mut(mesh).unwrap();

        *mesh = Mesh::from(LineStrip { points });
    }
}
