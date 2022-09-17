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
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
