#![allow(unused_imports)]

use std::sync::Mutex;

use bevy::{
    app::App,
    core::*,
    core_pipeline::{self, ClearColor},
    ecs::prelude::*,
    math::*,
    pbr2::{AmbientLight, PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::{Assets, Transform},
    render2::{
        camera::PerspectiveCameraBundle,
        color::Color,
        mesh::{shape, Mesh},
        options::*,
        texture::BevyDefault,
        view::ExtractedWindows,
        RenderApp, RenderStage,
    },
    window::{WindowDescriptor, WindowId},
};
use engine::prelude::*;
use rand::Rng;

pub fn run() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Boid".to_string(),
        // mode: bevy::window::WindowMode::Fullscreen,
        ..Default::default()
    })
    .insert_resource(WgpuOptions {
        features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | Features::CLEAR_COMMANDS,
        ..Default::default()
    })
    .add_plugin(EnginePlugin)
    .insert_inspector_resource::<ClearColor>(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
    .insert_inspector_resource::<AmbientLight>(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    })
    .add_startup_system(setup)
    .run();
    // let render_app = app.sub_app(RenderApp);
    // render_app.add_system_to_stage(RenderStage::Extract, time_extract_system);
    // render_app.init_resource::<BoidShaders>();
    // let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
    // graph.add_node(
    //     "mold",
    //     MoldNode {
    //         inner: Mutex::new(MoldNodeInner {
    //             time: 0.,
    //             state: ReadState::A,
    //         }),
    //     },
    // );
    // graph
    //     .add_node_edge(core_pipeline::node::MAIN_PASS_DRIVER, "mold")
    //     .unwrap();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(CameraController::default())
        .insert(Name::new("camera"));

    // Cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., -5.)),
            ..Default::default()
        })
        .insert(Name::new("cube"));

    // Light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 25.0,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(Name::new("light"));
}
