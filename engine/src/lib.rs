#![allow(clippy::type_complexity)]

mod camera_controller;
mod editor;
mod loaders;
mod materials;
mod shapes;

use bevy::{
    diagnostic::*,
    input::Input,
    prelude::{App, EventWriter, KeyCode, Plugin, Res, ResMut},
    render2::view::Msaa,
    PipelinedDefaultPlugins,
    window::{WindowMode, Windows},
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use camera_controller::CameraControllerPlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use editor::*;
use materials::MaterialPlugin;
use shapes::ShapePlugin;

pub mod prelude {
    pub use crate::{
        camera_controller::*, editor::*, loaders::*, materials::*, shapes::*,
        EnginePlugin,
    };
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {

        app.add_plugins(PipelinedDefaultPlugins)
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
            .add_plugin(MaterialPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            //.add_plugin(LogDiagnosticsPlugin::default());
            .add_system(control_system);

        //#[cfg(not(target_arch = "wasm32"))]
        // app.add_plugin(config::ConfigPlugin);
    }
}

fn control_system(
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<AppExit>,
    key_input: Res<Input<KeyCode>>,    
    mut windows: ResMut<Windows>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    {
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


}
