#![allow(clippy::type_complexity)]
#![allow(warnings)]

mod camera_controller;
mod editor;
mod loaders;
mod shapes;

use bevy::{diagnostic::*, prelude::*, window::{WindowMode, Windows}, asset::AssetServerSettings };
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use camera_controller::CameraControllerPlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;

// #[cfg(target_arch = "wasm32")]
// use bevy::{
//     asset::AssetServerSettings,
//     pbr::DirectionalLightShadowMap,
// };


pub mod prelude {
    pub use crate::{
        camera_controller::*, editor::*, loaders::*, shapes::*,
        EnginePlugin,
    };
}

pub struct EnginePlugin {
    pub title: String,
}

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {

        #[cfg(target_arch = "wasm32")]
        // Set asset path
        app.insert_resource(AssetServerSettings {
            asset_folder: format!( "/wasm/{}/assets",
            self.title.to_lowercase())
        });

        app.insert_resource(WindowDescriptor {
            title: self.title.clone(),
            #[cfg(target_arch = "wasm32")]
            width: 1024.,
            #[cfg(target_arch = "wasm32")]
            height: 800.,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("canvas.wasm".to_string()),
            //mode: WindowMode::Fullscreen,
            ..Default::default()
        })        
        .add_plugins(DefaultPlugins) // TODO: Move this back into each crate
        // TODO: this is not working
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(editor::EditorPlugin)
        .add_plugin(CameraControllerPlugin)
        //.add_plugin(ShapePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin);
        //.add_plugin(LogDiagnosticsPlugin::default());

        // .insert_resource(DirectionalLightShadowMap { size: 2048 })
        // //Dont think this works
        // 
        // .insert_resource(WgpuOptions {
        //     features: WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
        //         | WgpuFeatures::CLEAR_COMMANDS,
        //     ..Default::default()
        // });



        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(control_system);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn control_system(
    key_input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut exit: EventWriter<AppExit>,
) {
    if key_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    if key_input.just_pressed(KeyCode::F11) {
        let primary = windows.get_primary_mut().unwrap();
        if primary.mode() == WindowMode::Windowed {
            primary.set_mode(WindowMode::Fullscreen);
        } else {
            primary.set_mode(WindowMode::Windowed);
        }
    }
}
