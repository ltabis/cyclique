mod components;
mod event;
mod state;
mod systems;

use bevy::prelude::*;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup)
            .add_event::<event::Event>()
            .init_resource::<state::State>()
            .add_system(systems::update)
            .add_system(systems::new_body)
            .add_system(systems::update_bodies_velocity)
            .add_system(systems::update_bodies_position);
    }
}
