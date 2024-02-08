use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.) // defaults to 1kg
    }
}

#[derive(Debug, Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

#[derive(Component, Debug, Default)]
pub struct Vel(pub(crate) Vec2);

#[derive(Component, Debug, Default)]
pub struct PreSolveVel(pub(crate) Vec2);

#[derive(Default, Debug,Resource)]
pub struct Contacts(pub Vec<(Entity, Entity)>);

#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(1.) // perfectly elastic body
    }
}