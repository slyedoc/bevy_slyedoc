use bevy::prelude::*;

use bevy_mod_picking::{PickingCameraBundle, PickableBundle, DefaultPickingPlugins};
use engine::prelude::*;

fn main() {
    App::new()
        .add_plugin(EnginePlugin {
            title: "Sponza".to_string(),
        })
        
        .add_plugin(DefaultPickingPlugins)
        .init_inspector_resource::<ClearColor>()
        .insert_inspector_resource::<AmbientLight>(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(SceneInstance::default())
        .add_state(AppState::Loading)
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_system(scene_update)
        .run();
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    Playing,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    let instance_id =
        scene_spawner.spawn(asset_server.load("models/Sponza/glTF/Sponza.gltf#Scene0"));
    scene_instance.0 = Some(instance_id);

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        point_light: PointLight {
            intensity: 6000.0,
            range: 60.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Light - Point"));
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.5, 0.0),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert_bundle(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
}


// Resource to hold the scene `instance_id` until it is loaded
#[derive(Default)]
struct SceneInstance(Option<bevy::scene::InstanceId>);

fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    scene_instance: Res<SceneInstance>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Some(instance_id) = scene_instance.0 {
            if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
                entity_iter.for_each(|entity| {
                    commands
                        .entity(entity)
                        .insert_bundle(PickableBundle::default());
                });
                *done = true;
            }
        }
    }
}