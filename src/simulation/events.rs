use bevy::prelude::*;

use crate::game_state::GameState;

pub fn simulation_events(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        switch_simulation_state(&mut state);
    }
}

pub fn switch_simulation_state(state: &mut ResMut<State<GameState>>) {
    if let Some(error) = match state.current() {
        GameState::Paused => state.set(GameState::Run),
        GameState::Run => state.set(GameState::Paused),
    }
    .err()
    {
        error!("{error}");
    }
}
