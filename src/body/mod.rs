pub mod components;
mod events;
mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use self::components::{Body, Velocity};
use crate::{
    game_state::GameState,
    simulation::orbit_visualizer::{lines::LineMaterial, OrbitVisualizer},
};

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_startup_system(setup);

        app.register_inspectable::<Velocity>()
            .register_inspectable::<Body>()
            .add_system(events::body_events)
            .add_system(systems::new_body)
            .add_system_set(
                SystemSet::on_update(GameState::Run).with_system(systems::update_bodies),
            );
    }
}

#[cfg(debug_assertions)]
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    {
        let orbit = OrbitVisualizer::new(200, &mut commands, &mut meshes, &mut line_materials);

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 1.0,
                    subdivisions: 30,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..default()
            })
            .insert(Body {
                mass: 100000.0,
                fixed: true,
            })
            .insert(Velocity::default())
            .commands()
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 30,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(5.0, 5.0, 5.0),
                ..default()
            })
            .insert(Body {
                mass: 5.0,
                fixed: false,
            })
            .insert(orbit)
            .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)));
    }
}
