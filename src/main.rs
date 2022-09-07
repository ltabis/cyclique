mod body;

use bevy::prelude::*;
use body::BodyPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(BodyPlugin {})
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle::default());
}
