use bevy::prelude::*;

pub struct Settings {
    pub debug: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            debug: cfg!(debug_assertions),
        }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
    }
}
