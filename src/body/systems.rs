use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::simulation::math::{compute_acceleration, compute_single_step_world_accelerations};

use super::{
    components::{Body, Velocity},
    event::Event,
};

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
struct CelestialBodyQuery {
    entity: Entity,
    body: &'static Body,
    transform: &'static Transform,
    velocity: &'static Velocity,
}

pub fn update_bodies(
    time: Res<Time>,
    state: Res<super::state::State>,
    mut q_body: Query<(Entity, &Body, &mut Transform, &mut Velocity)>,
) {
    // TODO: replace state by Bevy Stages: https://docs.rs/bevy/0.8.1/bevy/ecs/prelude/trait.Stage.html
    if state.paused {
        return;
    }

    let mut accelerations = vec![];

    for (entity, body, transform, _) in &q_body {
        if body.fixed {
            continue;
        }

        let mut acceleration = Vec3::default();

        for (other, o_body, o_transform, _) in &q_body {
            if entity == other {
                continue;
            }

            acceleration += compute_acceleration(body, transform, o_body, o_transform);
        }

        accelerations.push((entity, acceleration));
    }

    // Update orbits with current velocities.
    for (entity, acceleration) in accelerations {
        let (_, _, mut transform, mut velocity) = q_body.get_mut(entity).unwrap();

        velocity.0 += acceleration;
        transform.translation += velocity.0;
    }
}

// TODO: use bundles to create bodies.
/// Create a new body if the click event has been fired.
pub fn new_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<Event>,
) {
    for event in events.iter() {
        match event {
            Event::NewBody => {
                debug!("new body created");
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 1.0,
                            subdivisions: 30,
                        })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        ..default()
                    })
                    .insert(Body {
                        mass: 5.0,
                        fixed: false,
                    });
            }
            _ => (),
        }
    }
}
