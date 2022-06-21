use bevy::prelude::*;

use crate::{components::Particle, utils::move_to_point};

pub fn red_particles_update_system(
    time: Res<Time>,
    mut particles_query: Query<(&Particle, &mut Transform)>,
) {
    let mut combinations = particles_query.iter_combinations_mut();
    while let Some([(p1, mut t1), (p2, t2)]) = combinations.fetch_next() {
        if p1.color == Color::RED {
            let distance = ((t2.translation.x - t1.translation.x)
                * (t2.translation.x - t1.translation.x)
                + (t2.translation.y - t1.translation.y) * (t2.translation.y - t1.translation.y))
                .sqrt();

            if distance < 300. && p2.color == Color::BLUE {
                t1.translation = move_to_point(t1.translation, t2.translation, 150. / distance * time.delta_seconds());
            } else if distance < 300. && p2.color == Color::RED {
                t1.translation = move_to_point(t1.translation, t2.translation, -150. / distance * time.delta_seconds());
            }
        }
    }
}
