use bevy::prelude::*;

use super::{components::Body, event::Event};

pub fn setup(_: Commands) {}

pub fn update(keyboard_input: Res<Input<KeyCode>>, mut events: EventWriter<Event>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        info!("event emited");
        events.send(Event::Click);
    }
}

pub fn new_body(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<Event>,
) {
    for event in events.iter() {
        match event {
            Event::Click => {
                info!("new body created");
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
        }
    }
}
