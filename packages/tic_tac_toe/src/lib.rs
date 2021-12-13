mod board;

use bevy::{
    core::prelude::*,
    core_pipeline::ClearColor,
    ecs:: prelude::*,
    math::*,
    pbr2::*,
    prelude::{ App, Assets, Commands, CoreStage, Handle, Transform},
    render2::{camera::*, color::Color},
    window::{CursorMoved, WindowDescriptor},
};
use bevy_inspector_egui::Inspectable;
use bevy_mod_raycast::*;
use board::*;
use engine::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugin(EnginePlugin)
        .insert_resource(WindowDescriptor {
            title: "Tic-Tac-Toe".to_string(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some("canvas.wasm".to_string()),
            ..Default::default()
        })
        .insert_inspector_resource::<ClearColor>(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_inspector_resource::<AmbientLight>(AmbientLight {
            color: Color::WHITE,
            brightness: 0.02,
        })
        .init_inspector_resource::<GameConfig>()
        .add_state(AppState::Loading)
        .add_plugin(DefaultRaycastingPlugin::<LocationRaycastSet>::default())
        // You will need to pay attention to what order you add systems! Putting them in the wrong
        // order can result in multiple frames of latency. Ray casting should probably happen after
        // the positions of your meshes have been updated in the UPDATE stage.
        .add_system_to_stage(
            CoreStage::PreUpdate,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays),
        )
        .add_startup_system(setup_camera)
        .add_startup_system(setup_board)
        .run();
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    Start,
    Playing,
    GameOver,
}

#[derive(Inspectable)]
pub struct GameConfig {
    #[inspectable(min = 0.0, max = 10.0)]
    board_size: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    grid_line_width: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    cell_fill: f32,
    line_material: Handle<StandardMaterial>,
    cell_material: Handle<StandardMaterial>,
}

impl FromWorld for GameConfig {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        Self {
            board_size: 3.0,
            grid_line_width: 0.1,
            cell_fill: 0.8,
            line_material: materials.add(StandardMaterial {
                base_color: Color::rgb_linear(0.5, 0.5, 0.5),
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
            cell_material: materials.add(StandardMaterial {
                base_color: Color::rgb_linear(0.8, 0.1, 0.1),
                perceptual_roughness: 1.0,
                unlit: true,
                ..Default::default()
            }),
        }
    }
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(RayCastSource::<LocationRaycastSet>::new())
        .insert(Name::new("Camera"));
}

// This is a unit struct we will use to mark our generic `RayCastMesh`s and `RayCastSource` as part
// of the same group, or "RayCastSet". For more complex use cases, you might use this to associate
// some meshes with one ray casting source, and other meshes with a different ray casting source."
pub struct LocationRaycastSet;

// Update our `RayCastSource` with the current cursor position every frame.
fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<LocationRaycastSet>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}
