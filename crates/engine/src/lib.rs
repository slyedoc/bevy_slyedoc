#![allow(clippy::type_complexity)]

mod camera_controller;
mod editor;
mod loaders;
mod shapes;

use bevy::{diagnostic::*, prelude::*, };
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use camera_controller::CameraControllerPlugin;

use editor::EditorPlugin;
use shapes::ShapePlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    app::AppExit,
    window::{WindowMode, Windows},
};

#[cfg(target_arch = "wasm32")]
use bevy::{
    asset::AssetServerSettings,
    pbr::DirectionalLightShadowMap,
};


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
        {
            app.insert_resource(WindowDescriptor {
                title: self.title.clone(),
                width: 600.,
                height: 400.,
                canvas: Some("canvas.wasm".to_string()),
                ..Default::default()
            })
                .insert_resource(DirectionalLightShadowMap { size: 2048 })
                //Dont think this works
                .insert_resource(AssetServerSettings {
                    asset_folder: format!( "{}/assets",  self.title.to_lowercase()),
                });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            app.insert_resource(WindowDescriptor {
                title: self.title.clone(),
                // width: 600.,
                // height: 400.,
                ..Default::default()
            });

        }

        app.add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WorldInspectorParams {
            // TODO: Figure out why egui crashes when false
            enabled: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(EditorPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin);
        //.add_plugin(LogDiagnosticsPlugin::default());

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(control_system);

        //#[cfg(not(target_arch = "wasm32"))]
        // app.add_plugin(config::ConfigPlugin);
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
