use bevy::prelude::*;

use crate::body::components::{Body, Velocity};

///
const GRAVITATIONAL_CONSTANT: f32 = 0.00001;

/// Compute accelerations of all provided bodies for a single step.
pub fn compute_single_step_world_accelerations(
    q_body: &Query<(Entity, &Body, &mut Transform, &mut Velocity)>,
) -> Vec<(Entity, Vec3)> {
    // FIXME: terrible memory stuff. Cannot know the nb of bodies since queries are iterators.
    let mut accelerations = vec![];

    for (entity, body, _, _) in q_body.iter() {
        if body.fixed {
            continue;
        }

        accelerations.push((entity, compute_all_accelerations(&entity, &q_body)));
    }

    accelerations
}

/// Get the acceleration of an entity compared to a group of entities.
/// (hopefully the world around the entity)
///
/// # Args
/// * `entity` - The entity to compute the acceleration for.
/// * `other`  - The set of bodies that will be used to compute the acceleration of `entity`.
/// * `q_body` - A Bevy query used to retrieve bodies components.
pub fn compute_all_accelerations(
    entity: &Entity,
    // FIXME: `&mut Velocity` and `&mut Transform` are used here because the previous function needs the
    //        component to be mutable.
    q_body: &Query<(Entity, &Body, &mut Transform, &mut Velocity)>,
) -> Vec3 {
    // FIXME: terrible.
    let mut acceleration = Vec3::default();

    let (_, body, transform, _) = q_body
        .get(*entity)
        .expect("body/transform components are missing from a body");

    for (other, other_body, other_transform, _) in q_body {
        if *entity == other {
            continue;
        }

        acceleration += compute_acceleration(body, transform, other_body, other_transform);
    }

    acceleration
}

/// Compte acceleration for a body compared to another.
pub fn compute_acceleration(
    body: &Body,
    transform: &Transform,
    other_body: &Body,
    other_transform: &Transform,
) -> Vec3 {
    let diff = other_transform.translation - transform.translation;
    let sqr_dist = diff.length().powf(2.0);
    let force_dir = diff.normalize();
    let force = force_dir * GRAVITATIONAL_CONSTANT * body.mass * other_body.mass / sqr_dist;

    force / body.mass // body.mass cancels out, it's just here for clarity.
}
