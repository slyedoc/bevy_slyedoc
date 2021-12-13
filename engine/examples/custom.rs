use bevy::{
    core::Name,
    math::Vec3,
    pbr2::*,
    prelude::{info, App, Assets, Commands, ResMut, Transform},
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
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("setup cube");
    // cube
    commands
        .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(StandardMaterial {
             base_color: Color::GREEN,
             ..Default::default()
        }),
        ..Default::default()
        // materials.add(CustomMaterial {
        //     color: Color::GREEN,
        // }),
    })
        .insert(Name::new("Cube"));

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera"));

    // light
    commands
        .spawn_bundle(DirectionalLightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}
