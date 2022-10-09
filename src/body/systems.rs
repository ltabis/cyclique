use bevy::prelude::*;

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
            .insert(Body { mass: 50.0 })
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
            .insert(Body { mass: 5.0 })
            .insert(Velocity::default());
    }
}

const GRAVITATIONAL_CONSTANT: f32 = 1.0;

pub fn update_bodies_velocity(
    time: Res<Time>,
    state: Res<super::state::State>,
    bodies: Query<Entity, With<Body>>,
    characteristics: Query<(&Body, &Transform)>,
    mut velocities: Query<&mut Velocity, With<Body>>,
) {
    if state.paused {
        return;
    }

    for body in &bodies {
        let mut acceleration = Vec3::default();

        debug!("Updating body {:?}", body);

        for other in &bodies {
            if body == other {
                continue;
            }

            let (body, transform) = characteristics.get(body).unwrap();
            let (other, other_transform) = characteristics.get(other).unwrap();

            let diff = other_transform.translation - transform.translation;
            let sqr_dist = diff.length().powf(2.0);
            let force_dir = diff.normalize();
            let force = force_dir * GRAVITATIONAL_CONSTANT * body.mass * other.mass / sqr_dist;

            acceleration += force / body.mass;
        }

        debug!("Acceleration: {:?}", acceleration);

        // updating the current body's velocity.
        if let Err(error) = velocities
            .get_mut(body)
            .map(|mut velocity| velocity.0 += acceleration * time.delta_seconds())
        {
            error!("Failed to update velocity: {}", error);
        }
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
                    .insert(Body { mass: 5.0 });
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
