use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::simulation::math::compute_single_step_world_accelerations;

use super::components::{Body, Velocity};

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
struct CelestialBodyQuery {
    entity: Entity,
    body: &'static Body,
    transform: &'static Transform,
    velocity: &'static Velocity,
}

pub fn update_bodies(
    _: Res<Time>,

    mut q_body: Query<(Entity, &Body, &mut Transform, &mut Velocity)>,
) {
    let accelerations = compute_single_step_world_accelerations(&q_body);

    // Update orbits with current velocities.
    for (entity, acceleration) in accelerations {
        let (_, _, mut transform, mut velocity) = q_body.get_mut(entity).unwrap();

        // NOTE: should be `velocity.0 += acceleration * time.delta_seconds();` but results
        //       do not match the orbit visualizer because of the slight variation of values.
        velocity.0 += acceleration;
        transform.translation += velocity.0;
    }
}
