pub mod components;
mod event;
mod state;
mod systems;

use bevy::prelude::*;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup)
            .add_plugin(MaterialPlugin::<
                crate::simulation::orbit_visualizer::lines::LineMaterial,
            >::default())
            .add_event::<event::Event>()
            .init_resource::<state::State>()
            .init_resource::<crate::simulation::orbit_visualizer::OrbitVisualizer>()
            .add_system(systems::update)
            .add_system(systems::new_body)
            .add_system(systems::update_bodies_velocity)
            .add_system(systems::update_bodies_position)
            .add_system(crate::simulation::display_orbits);
    }
}
