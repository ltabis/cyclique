use bevy::prelude::*;

use crate::events::Event;

pub fn control_events(keyboard_input: Res<Input<KeyCode>>, mut events: EventWriter<Event>) {
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]) {
        events.send(Event::Move(
            keyboard_input.get_pressed().next().unwrap().clone(),
        ));
    }
}
