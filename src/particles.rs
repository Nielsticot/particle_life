use bevy::{prelude::*, utils::HashMap};
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

use crate::components::{Particle, ParticleRules, Velocity};

pub struct ParticlesPlugin;

const PARTICLE_NUMBER: u32 = 150;
const PARTICLE_SIZE: f32 = 5.;
const PARTICLE_RANGE: f32 = 30.;
const PARTICLE_SPEED: f32 = 19.;
const FRICTION: f32 = 5.;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(rules_setup_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, particles_spawn_system)
            .add_system(particles_interaction_system)
            .add_system(particles_friction_system)
            .add_system(move_particles_system)
            .add_system(world_border_system);
    }
}

fn rules_setup_system(mut commands: Commands) {
    commands.insert_resource(ParticleRules {
        interactions: HashMap::from_iter([
            (
                Particle::RED,
                HashMap::from_iter([(Particle::BLUE, 1.), (Particle::GREEN, 0.5)]),
            ),
            (
                Particle::BLUE,
                HashMap::from_iter([(Particle::RED, -0.5), (Particle::GREEN, 0.8)]),
            ),
            (
                Particle::GREEN,
                HashMap::from_iter([(Particle::GREEN, -0.1), (Particle::RED, 0.3), (Particle::BLUE, 1.)]),
            ),
        ]),
    });
}

fn particles_spawn_system(mut commands: Commands, windows: Res<Windows>) {
    let mut rng = thread_rng();
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());

    let shape = shapes::Circle {
        radius: PARTICLE_SIZE,
        center: Vec2::new(0., 0.),
    };

    for _ in 0..PARTICLE_NUMBER {
        let particle_type = match rng.gen_range(0..=2) {
            0 => Particle::RED,
            1 => Particle::GREEN,
            _ => Particle::BLUE,
        };

        let color = match particle_type {
            Particle::RED => Color::RED,
            Particle::GREEN => Color::GREEN,
            Particle::BLUE => Color::BLUE,
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(color)),
                Transform::from_translation(Vec3::new(
                    (rng.gen::<f32>() - 0.5) * width / 2.,
                    (rng.gen::<f32>() - 0.5) * height / 2.,
                    0.,
                )),
            ))
            .insert(Velocity { value: Vec3::ZERO })
            .insert(particle_type);
    }
}

fn particles_interaction_system(
    time: Res<Time>,
    rules: ResMut<ParticleRules>,
    mut particles_query: Query<(&Particle, &Transform, &mut Velocity)>,
) {
    let mut combinations = particles_query.iter_combinations_mut();
    while let Some([(p1, t1, mut v1), (p2, t2, mut v2)]) = combinations.fetch_next() {
        let dx = t2.translation.x - t1.translation.x;
        let dy = t2.translation.y - t1.translation.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if rules.interactions.contains_key(p1) && rules.interactions[p1].contains_key(p2) {
            let direction = (t2.translation - t1.translation).normalize();

            if distance < PARTICLE_SIZE * 2. {
                v1.value = direction * -5. * PARTICLE_SPEED * time.delta_seconds();
            } else if distance < PARTICLE_RANGE {
                v1.value = v1.value
                    + direction
                        * PARTICLE_SPEED
                        * time.delta_seconds()
                        * rules.interactions[p1][p2]
                        / (distance / PARTICLE_RANGE);
            }
        }

        if rules.interactions.contains_key(p2) && rules.interactions[p2].contains_key(p1) {
            let direction = (t1.translation - t2.translation).normalize();

            if distance < PARTICLE_SIZE * 2. {
                v2.value = direction * -5. * PARTICLE_SPEED * time.delta_seconds();
            } else if distance < PARTICLE_RANGE {
                v2.value = v2.value
                    + direction
                        * PARTICLE_SPEED
                        * time.delta_seconds()
                        * rules.interactions[p2][p1]
                        / (distance / PARTICLE_RANGE);
            }
        }
    }
}

fn particles_friction_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Particle>>) {
    for mut velocity in query.iter_mut() {
        velocity.value.x -= FRICTION * time.delta_seconds();
        velocity.value.y -= FRICTION * time.delta_seconds();

        if velocity.value.x < 0. {
            velocity.value.x = 0.;
        }
        if velocity.value.y < 0. {
            velocity.value.y = 0.;
        }
    }
}

fn move_particles_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Particle>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value * PARTICLE_SPEED * time.delta_seconds();
    }
}

fn world_border_system(mut query: Query<(&mut Transform), With<Particle>>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());

    for mut transform in query.iter_mut() {
        if transform.translation.x < -width / 2. {
            transform.translation.x = width / 2.;
        }
        if transform.translation.x > width / 2. {
            transform.translation.x = -width / 2.;
        }
        if transform.translation.y < -height / 2. {
            transform.translation.y = height / 2.;
        }
        if transform.translation.y > height / 2. {
            transform.translation.y = -height / 2.;
        }
    }
}
