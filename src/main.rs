mod body;

use bevy::prelude::*;
use body::BodyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(BodyPlugin {})
        // Camera
        .add_plugin(smooth_bevy_cameras::LookTransformPlugin)
        .add_plugin(smooth_bevy_cameras::controllers::orbit::OrbitCameraPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(
            smooth_bevy_cameras::controllers::orbit::OrbitCameraBundle::new(
                smooth_bevy_cameras::controllers::orbit::OrbitCameraController::default(),
                Vec3::new(-2.0, 5.0, 5.0),
                Vec3::new(0., 0., 0.),
            ),
        );
}
