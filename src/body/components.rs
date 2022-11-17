use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Debug)]
pub struct Body {
    /// The mass of the object.
    pub mass: f32,
    /// A hack to make the body not move.
    pub fixed: bool,
}

#[derive(Component, Default, Inspectable, Clone, Copy, Debug)]
pub struct Velocity(pub Vec3);
