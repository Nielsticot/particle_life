use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use particles::ParticlesPlugin;

mod components;
mod particles;
mod utils;

const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_plugin(ParticlesPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
