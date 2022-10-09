use bevy::prelude::*;

#[derive(Component)]
pub struct Body {
    pub mass: f32,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);
