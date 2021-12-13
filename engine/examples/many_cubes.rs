use bevy::{
    ecs::prelude::*,
    pbr2::{PbrBundle, StandardMaterial},
    prelude::{App, Assets, Transform},
    render2::{
        camera::PerspectiveCameraBundle,
        color::Color,
        mesh::{shape, Mesh},
    },
};
use engine::prelude::*;

fn main() {
    App::new()
        .add_plugin(EnginePlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // cube
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    base_color: Color::PINK,
                    ..Default::default()
                }),
                transform: Transform::from_xyz((x as f32) * 2.0, (y as f32) * 2.0, 0.0),
                ..Default::default()
            });
        }
    }

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(80.0, 80.0, 300.0),
            ..Default::default()
        })
        .insert(CameraController::default());
}
