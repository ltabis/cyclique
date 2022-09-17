use bevy::prelude::*;

pub enum Event {
    NewBody,
    Move(KeyCode),
}
