use bevy::prelude::*;

use crate::body::components::{Body, Velocity};

///
const GRAVITATIONAL_CONSTANT: f32 = 1.0;

/// Get the acceleration of an entity compared to a group of entities.
/// (hopefully the world around the entity)
///
/// # Args
/// * `entity` - The entity to compute the acceleration for.
/// * `other`  - The set of bodies that will be used to compute the acceleration of `entity`.
/// * `q_body` - A Bevy query used to retrieve bodies components.
pub fn compute_acceleration(
    entity: &Entity,
    others: &Query<Entity, With<Body>>,
    q_body: &Query<(&Body, &Transform, &mut Velocity)>,
) -> Vec3 {
    let mut acceleration = Vec3::default();

    let (body, transform, _) = q_body
        .get(*entity)
        .expect("body/transform components are missing from a body");

    for other in others {
        if *entity == other {
            continue;
        }

        let (other, other_transform, _) = q_body
            .get(other)
            .expect("body/transform components are missing from a body");

        let diff = other_transform.translation - transform.translation;
        let sqr_dist = diff.length().powf(2.0);
        let force_dir = diff.normalize();
        let force = force_dir * GRAVITATIONAL_CONSTANT * body.mass * other.mass / sqr_dist;

        acceleration += force / body.mass;
    }

    acceleration
}
