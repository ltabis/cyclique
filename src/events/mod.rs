use bevy::prelude::*;

pub enum Event {
    NewBody(Transform),
    Move(KeyCode),
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Event>();
    }
}
