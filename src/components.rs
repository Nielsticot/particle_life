use bevy::{math::Vec3, prelude::Component, utils::HashMap};

#[derive(Component, Eq, Hash, PartialEq)]
pub enum Particle {
    RED,
    GREEN,
    BLUE,
}

#[derive(Component)]
pub struct ParticleRules {
    pub interactions: HashMap<Particle, HashMap<Particle, f32>>,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}
