pub mod components;
mod inputs;
pub mod plugin;
mod systems;

use bevy::prelude::*;

use self::components::{Body, Velocity};
use crate::simulation::orbit_visualizer::{
    lines::{LineMaterial, LineStrip},
    OrbitVisualizer,
};

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

fn new_body(
    transform: Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    line_materials: &mut ResMut<Assets<LineMaterial>>,
) {
    let orbit = OrbitVisualizer::new(200, commands, meshes, line_materials);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                subdivisions: 30,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform,
            ..default()
        })
        .insert(Body {
            mass: 5.0,
            fixed: false,
        })
        .insert(orbit)
        .insert(Velocity(Vec3::new(0.0, 0.0, 0.0)))
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .add_children(|builder| {
            builder.spawn().insert_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(LineStrip {
                    points: vec![Vec3::ZERO, Vec3::new(10., 0., 0.)],
                })),
                material: line_materials.add(LineMaterial { color: Color::BLUE }),
                ..default()
            });
            builder.spawn().insert_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(LineStrip {
                    points: vec![Vec3::ZERO, Vec3::new(0., 10., 0.)],
                })),
                material: line_materials.add(LineMaterial { color: Color::RED }),
                ..default()
            });
            builder.spawn().insert_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(LineStrip {
                    points: vec![Vec3::ZERO, Vec3::new(0., 0., 10.)],
                })),
                material: line_materials.add(LineMaterial {
                    color: Color::GREEN,
                }),
                ..default()
            });
        });
}
