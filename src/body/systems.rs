use bevy::prelude::*;

use crate::simulation::math::compute_acceleration;

use super::{
    components::{Body, Velocity},
    event::Event,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    #[cfg(debug_assertions)]
    {
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
                mass: 50.0,
                fixed: true,
            })
            .insert(Velocity::default())
            .commands()
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 30,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(5.0, 5.0, 5.0),
                ..default()
            })
            .insert(Body {
                mass: 5.0,
                fixed: false,
            })
            .insert(Velocity(Vec3::new(0.1, 0.0, 0.0)));
    }
}

pub fn update_bodies_velocity(
    time: Res<Time>,
    state: Res<super::state::State>,
    bodies: Query<Entity, With<Body>>,
    mut q_body: Query<(&Body, &Transform, &mut Velocity)>,
) {
    if state.paused {
        return;
    }

    for entity in &bodies {
        let body = if let Ok((body, _, _)) = q_body.get(entity) {
            body
        } else {
            error!("celestial body does not contain Body / Transform / Velocity components");
            continue;
        };

        if body.fixed {
            debug!("Body {entity:?} is fixed, not updating.");
            continue;
        }

        let acceleration = compute_acceleration(&entity, &bodies, &q_body);
        debug!("Acceleration: {:?}", acceleration);

        if let Ok((_, _, mut velocity)) = q_body.get_mut(entity) {
            // updating the current body's velocity.
            velocity.0 += acceleration * time.delta_seconds();
        } else {
            error!("celestial body does not contain Body / Transform / Velocity components");
            continue;
        };
    }
}

pub fn update_bodies_position(
    state: Res<super::state::State>,
    mut bodies: Query<(&mut Transform, &Velocity), With<Body>>,
) {
    if state.paused {
        return;
    }

    for (mut transform, velocity) in &mut bodies {
        transform.translation += velocity.0;
    }
}

pub fn update(
    mut state: ResMut<super::state::State>,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<Event>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        debug!("new body event");
        events.send(Event::NewBody);
    } else if keyboard_input.any_pressed([
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
    ]) {
        events.send(Event::Move(
            keyboard_input.get_pressed().next().unwrap().clone(),
        ));
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        state.paused = !state.paused;
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

// const CAMERA_SPEED: f32 = 5.0;

// TODO: re-invent the wheel by coding my own camera controller.
// /// Control the camera using the keyboard.
// ///
// #[bevy::utils::tracing::instrument(skip_all)]
// pub fn camera_controller(
//     mut camera: Query<&mut Transform, With<Camera>>,
//     mut events: EventReader<Event>,
//     time: Res<Time>,
// ) {
//     for event in events.iter() {
//         match event {
//             Event::Move(key) => {
//                 if let Some(mut camera) = camera.iter_mut().next() {
//                     match key {
//                         KeyCode::Up => camera.translation.z += CAMERA_SPEED * time.delta_seconds(),
//                         KeyCode::Down => {
//                             camera.translation.z -= CAMERA_SPEED * time.delta_seconds()
//                         }
//                         KeyCode::Right => {
//                             camera.translation.x += CAMERA_SPEED * time.delta_seconds()
//                         }
//                         KeyCode::Left => {
//                             camera.translation.x -= CAMERA_SPEED * time.delta_seconds()
//                         }
//                         _ => {}
//                     }
//                 } else {
//                     error!("no camera found.");
//                 }
//             }
//             _ => {}
//         }
//     }
// }
