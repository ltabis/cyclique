use bevy::{ecs::query::WorldQuery, prelude::*};

use crate::simulation::math::compute_single_step_world_accelerations;

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

    let accelerations_for_one_step = compute_single_step_world_accelerations(&q_body);

    for result in &accelerations_for_one_step {
        if let Ok((_, _, mut transform, mut velocity)) = q_body.get_mut(result.0) {
            // updating the current body's velocity.
            velocity.0 += result.1 * time.delta_seconds();
            transform.translation += velocity.0;
        } else {
            error!("celestial body does not contain Body / Transform / Velocity components");
            continue;
        };
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
