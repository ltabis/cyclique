use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use super::components::{Body, Velocity};
use crate::game_state::GameState;

use super::{inputs, setup, systems};

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_startup_system(setup);

        app.register_inspectable::<Velocity>()
            .register_inspectable::<Body>()
            .add_system(inputs::body_inputs)
            .add_system_set(
                SystemSet::on_update(GameState::Run).with_system(systems::update_bodies),
            );
    }
}
