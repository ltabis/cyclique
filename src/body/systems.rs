use bevy::prelude::*;

use super::{components::Body, event::Event};

pub fn setup(_: Commands) {}

pub fn update(keyboard_input: Res<Input<KeyCode>>, mut events: EventWriter<Event>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        info!("event emited");
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
    }
}

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
                    .insert(Body);
            }
            _ => (),
        }
    }
}

const CAMERA_SPEED: f32 = 5.0;

/// Control the camera using the keyboard.
pub fn camera_controller(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut events: EventReader<Event>,
    time: Res<Time>,
) {
    for event in events.iter() {
        match event {
            Event::Move(key) => {
                let mut camera = camera.iter_mut().next().expect("No camera has been setup");

                match key {
                    KeyCode::Up => camera.translation.y += CAMERA_SPEED * time.delta_seconds(),
                    KeyCode::Down => camera.translation.y -= CAMERA_SPEED * time.delta_seconds(),
                    KeyCode::Right => camera.translation.x += CAMERA_SPEED * time.delta_seconds(),
                    KeyCode::Left => camera.translation.x -= CAMERA_SPEED * time.delta_seconds(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
