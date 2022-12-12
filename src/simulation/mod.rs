pub mod events;
pub mod math;
pub mod orbit_visualizer;

use crate::game_state::GameState;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(events::simulation_events)
            .add_plugin(MaterialPlugin::<orbit_visualizer::lines::LineMaterial>::default())
            .register_inspectable::<orbit_visualizer::OrbitVisualizer>()
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .with_system(crate::simulation::orbit_visualizer::systems::simulate_orbits),
            );
    }
}
