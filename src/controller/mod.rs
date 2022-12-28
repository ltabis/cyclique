use bevy::prelude::*;

use crate::game_state::GameState;

mod events;
mod ui;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui::setup)
            .add_system(ui::button_system)
            .add_system(ui::update_button_text)
            .add_system_set(
                SystemSet::on_update(GameState::Run).with_system(ui::update_button_text),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Paused).with_system(ui::update_button_text),
            )
            .add_system(events::control_events);
    }
}
