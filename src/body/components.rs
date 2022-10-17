use bevy::prelude::*;

#[derive(Component)]
pub struct Body {
    /// The mass of the object.
    pub mass: f32,
    /// A hack to make the body not move.
    pub fixed: bool,
}

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity(pub Vec3);
