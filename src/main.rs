mod body;
mod controller;
mod events;
mod game_state;
mod settings;
mod simulation;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(body::plugin::BodyPlugin {})
        .add_plugin(controller::ControllerPlugin {})
        .add_plugin(events::EventPlugin {})
        .add_plugin(simulation::SimulationPlugin {})
        .add_plugin(settings::SettingsPlugin {})
        .add_plugin(game_state::GameStatePlugin {})
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
        )
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default());
}
