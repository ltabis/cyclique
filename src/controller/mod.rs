use bevy::prelude::*;

mod events;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(events::control_events);
    }
}
