use bevy::prelude::*;

use super::state::GameState;

pub enum Event {
    NewBody,
    Move(KeyCode),
}

pub fn update(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<Event>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        debug!("new body event");
        events.send(Event::NewBody);
    } else if keyboard_input.any_pressed([
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
    ]) {
        events.send(Event::Move(
            keyboard_input.get_pressed().next().unwrap().clone(),
        ));
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(error) = match state.current() {
            GameState::Paused => state.set(GameState::Run),
            GameState::Run => state.set(GameState::Paused),
        }
        .err()
        {
            error!("{error}");
        }
    }
}
