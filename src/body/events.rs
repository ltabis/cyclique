use crate::events::Event;
use bevy::prelude::*;

pub fn body_events(keyboard_input: Res<Input<KeyCode>>, mut events: EventWriter<Event>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        // TODO: take mouse pos to 3D.
        events.send(Event::NewBody(Transform::default()));
    }
}
