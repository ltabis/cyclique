pub mod components;
mod event;
pub mod state;
mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use crate::simulation::orbit_visualizer::{lines::LineMaterial, OrbitVisualizer};

use self::components::{Body, Velocity};

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(MaterialPlugin::<
                crate::simulation::orbit_visualizer::lines::LineMaterial,
            >::default())
            .register_inspectable::<OrbitVisualizer>()
            .register_inspectable::<Velocity>()
            .register_inspectable::<Body>()
            .add_event::<event::Event>()
            .init_resource::<state::State>()
            .add_system(event::update)
            .add_system(systems::new_body)
            .add_system(systems::update_bodies)
            .add_system(crate::simulation::orbit_visualizer::systems::simulate_orbits);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    #[cfg(debug_assertions)]
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
